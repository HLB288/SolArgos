use super::models::{PerformanceSample, SolanaAnalysis, format_number};

// ===============================
// CALCULS D'ESTIMATION AVANCÉS
// ===============================

/// Calculer l'estimation finale 24h avec moyenne pondérée
pub fn calculate_final_estimate(
    performance_tx: u64,
    performance_hours: f64,
    performance_tps: f64,
    global_tx: u64,
    contracts_sample: u64,
) -> (u64, f64, f32) {
    
    // Méthode 1: Extrapolation Performance Samples
    let extrapolated_24h = if performance_hours > 0.0 {
        (performance_tx as f64 * (24.0 / performance_hours)) as u64
    } else {
        performance_tx
    };
    
    // Méthode 2: Estimation basée sur TPS observé
    let tps_based_24h = (performance_tps * 24.0 * 3600.0) as u64;
    
    // Méthode 3: Validation par échantillons
    // Les 3 contrats représentent environ 0.5% du trafic total
    let contracts_estimated = if contracts_sample > 0 {
        contracts_sample * 200
    } else {
        0
    };
    
    println!("\n🧮 CALCULS D'ESTIMATION:");
    println!("   Extrapolation temporelle: {}", format_number(extrapolated_24h));
    println!("   Basée sur TPS observé: {}", format_number(tps_based_24h));
    if contracts_estimated > 0 {
        println!("   Validation contrats: {}", format_number(contracts_estimated));
    }
    
    // Calcul de la moyenne pondérée
    let weighted_estimate = if contracts_estimated > 0 {
        (
            extrapolated_24h as f64 * 0.6 +  // 60% Performance Samples
            tps_based_24h as f64 * 0.3 +     // 30% TPS observé
            contracts_estimated as f64 * 0.1  // 10% validation contrats
        ) as u64
    } else {
        // Sans échantillons contrats, moyenne des 2 premières méthodes
        ((extrapolated_24h as f64 * 0.7 + tps_based_24h as f64 * 0.3) as u64)
    };
    
    let final_tps = weighted_estimate as f64 / (24.0 * 3600.0);
    
    // Score de confiance basé sur la cohérence des méthodes
    let confidence = calculate_confidence_score(
        extrapolated_24h,
        tps_based_24h,
        contracts_estimated,
    );
    
    (weighted_estimate, final_tps, confidence)
}

/// Calculer le score de confiance basé sur la variance des méthodes
fn calculate_confidence_score(
    extrapolated: u64,
    tps_based: u64,
    contracts: u64,
) -> f32 {
    let mut values = vec![extrapolated as f64, tps_based as f64];
    
    if contracts > 0 {
        values.push(contracts as f64);
    }
    
    if values.is_empty() {
        return 0.0;
    }
    
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    
    let std_dev = {
        let variance_sum: f64 = values.iter()
            .map(|value| {
                let diff = mean - value;
                diff * diff
            })
            .sum();
        (variance_sum / values.len() as f64).sqrt()
    };
    
    let coefficient_variation = if mean > 0.0 {
        std_dev / mean
    } else {
        1.0
    };
    
    // Plus le coefficient de variation est faible, plus la confiance est élevée
    (1.0 - coefficient_variation.min(1.0)).max(0.0) as f32
}

// ===============================
// DÉTECTION D'ANOMALIES
// ===============================

/// Détecter les anomalies dans les échantillons de performance
pub fn detect_anomalies(samples: &[PerformanceSample]) -> Vec<String> {
    let mut anomalies = Vec::new();
    
    if samples.len() < 2 {
        return anomalies;
    }
    
    // Calculer la moyenne TPS
    let total_tx: u64 = samples.iter().map(|s| s.num_transactions).sum();
    let total_time: u64 = samples.iter().map(|s| s.sample_period_secs).sum();
    let avg_tps = if total_time > 0 {
        total_tx as f64 / total_time as f64
    } else {
        0.0
    };
    
    if avg_tps == 0.0 {
        return anomalies;
    }
    
    // Détecter les pics ou chutes anormales
    for (i, sample) in samples.iter().enumerate() {
        let sample_tps = if sample.sample_period_secs > 0 {
            sample.num_transactions as f64 / sample.sample_period_secs as f64
        } else {
            0.0
        };
        
        // Pic anormal (>200% de la moyenne)
        if sample_tps > avg_tps * 2.0 {
            anomalies.push(format!(
                "🔺 Pic TPS détecté: {:.0} TPS (échantillon {}) - +{:.0}% vs moyenne",
                sample_tps, 
                i,
                ((sample_tps / avg_tps - 1.0) * 100.0)
            ));
        }
        
        // Chute anormale (<40% de la moyenne)
        if sample_tps < avg_tps * 0.4 && sample_tps > 0.0 {
            anomalies.push(format!(
                "🔻 Chute TPS détectée: {:.0} TPS (échantillon {}) - {:.0}% vs moyenne",
                sample_tps,
                i,
                ((1.0 - sample_tps / avg_tps) * 100.0)
            ));
        }
    }
    
    // Limiter le nombre d'anomalies reportées
    if anomalies.len() > 5 {
        anomalies.truncate(5);
        anomalies.push("... et plus d'anomalies détectées".to_string());
    }
    
    anomalies
}

// ===============================
// ANALYSE DE TENDANCES
// ===============================

/// Analyser les tendances sur la période observée
pub fn analyze_trends(samples: &[PerformanceSample]) -> String {
    if samples.len() < 10 {
        return "Données insuffisantes pour l'analyse de tendance".to_string();
    }
    
    // Diviser en deux moitiés pour comparer
    let mid = samples.len() / 2;
    let first_half = &samples[0..mid];
    let second_half = &samples[mid..];
    
    // Calculer TPS moyen pour chaque moitié
    let first_tps = calculate_avg_tps(first_half);
    let second_tps = calculate_avg_tps(second_half);
    
    if first_tps == 0.0 {
        return "TPS première période invalide".to_string();
    }
    
    let change_percent = ((second_tps - first_tps) / first_tps) * 100.0;
    
    if change_percent > 15.0 {
        format!("📈 Tendance haussière: +{:.1}% TPS", change_percent)
    } else if change_percent < -15.0 {
        format!("📉 Tendance baissière: {:.1}% TPS", change_percent)
    } else {
        format!("➡️ Tendance stable: {:.1}% TPS", change_percent)
    }
}

/// Calculer le TPS moyen d'un ensemble d'échantillons
fn calculate_avg_tps(samples: &[PerformanceSample]) -> f64 {
    let total_tx: u64 = samples.iter().map(|s| s.num_transactions).sum();
    let total_time: u64 = samples.iter().map(|s| s.sample_period_secs).sum();
    
    if total_time > 0 {
        total_tx as f64 / total_time as f64
    } else {
        0.0
    }
}

// ===============================
// ANALYSE COMPLÈTE
// ===============================

/// Créer une analyse complète avec toutes les métriques
pub fn create_complete_analysis(
    performance_tx: u64,
    performance_non_vote: u64,
    performance_tps: f64,
    performance_hours: f64,
    global_tx: u64,
    contracts_sample: u64,
    samples: &[PerformanceSample],
    analysis_time_ms: u128,
) -> SolanaAnalysis {
    
    // Calcul estimation finale
    let (final_estimate, final_tps, confidence) = calculate_final_estimate(
        performance_tx,
        performance_hours,
        performance_tps,
        global_tx,
        contracts_sample,
    );
    
    // Détection anomalies
    let anomalies = detect_anomalies(samples);
    
    // Analyse tendances
    let trend = analyze_trends(samples);
    
    SolanaAnalysis {
        performance_total_tx: performance_tx,
        performance_non_vote_tx: performance_non_vote,
        performance_tps,
        performance_period_hours: performance_hours,
        global_total_tx: global_tx,
        contracts_sample,
        final_estimate_24h: final_estimate,
        final_tps_avg: final_tps,
        confidence_score: confidence,
        analysis_time_ms,
        anomalies,
        trend,
    }
}

// ===============================
// AFFICHAGE DES RÉSULTATS
// ===============================

/// Afficher un résumé de l'analyse
pub fn display_analysis_summary(analysis: &SolanaAnalysis) {
    println!("\n🚀 ╔══ ANALYSE SOLANA 24H - RÉSULTATS FINAUX ══╗");
    println!("════════════════════════════════════════════════════");
    
    println!("📊 DONNÉES DE BASE:");
    println!("   Performance Samples: {} tx sur {:.1}h", 
             format_number(analysis.performance_total_tx), 
             analysis.performance_period_hours);
    println!("   TPS réel observé: {:.0}", analysis.performance_tps);
    println!("   Non-vote: {}", format_number(analysis.performance_non_vote_tx));
    println!("   Total historique: {}", format_number(analysis.global_total_tx));
    
    println!("\n🎯 ESTIMATION FINALE 24H:");
    println!("   Transactions totales: {}", format_number(analysis.final_estimate_24h));
    println!("   TPS moyen: {:.0}", analysis.final_tps_avg);
    println!("   Confiance: {:.0}%", analysis.confidence_score * 100.0);
    
    let performance_grade = if analysis.final_tps_avg > 4000.0 {
        "🟢 EXCELLENTE"
    } else if analysis.final_tps_avg > 3000.0 {
        "🟡 TRÈS BONNE" 
    } else if analysis.final_tps_avg > 2000.0 {
        "🟠 BONNE"
    } else {
        "🔴 MODÉRÉE"
    };
    
    println!("   Performance réseau: {}", performance_grade);
    
    println!("\n📈 TENDANCE:");
    println!("   {}", analysis.trend);
    
    if !analysis.anomalies.is_empty() {
        println!("\n⚠️ ANOMALIES DÉTECTÉES:");
        for anomaly in &analysis.anomalies {
            println!("   {}", anomaly);
        }
    }
    
    println!("\n⏱️ PERFORMANCE ANALYSE:");
    println!("   Temps de collecte: {}ms", analysis.analysis_time_ms);
    
    println!("════════════════════════════════════════════════════");
}