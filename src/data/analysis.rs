use super::{SolanaAnalysis, PerformanceSample};

// ===============================
// CALCULS D'ESTIMATION AVANCÉS
// ===============================

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
    let contracts_estimated = contracts_sample * 200; // Facteur ajusté
    
    println!("\n🧮 CALCULS D'ESTIMATION:");
    println!("   Extrapolation temporelle: {}", format_number(extrapolated_24h));
    println!("   Basée sur TPS observé: {}", format_number(tps_based_24h));
    println!("   Validation contrats: {}", format_number(contracts_estimated));
    
    // Calcul de la moyenne pondérée
    let weighted_estimate = (
        extrapolated_24h as f64 * 0.6 +  // 60% Performance Samples
        tps_based_24h as f64 * 0.3 +     // 30% TPS observé
        contracts_estimated as f64 * 0.1  // 10% validation contrats
    ) as u64;
    
    let final_tps = weighted_estimate as f64 / (24.0 * 3600.0);
    
    // Score de confiance basé sur la cohérence des méthodes
    let variance = vec![
        extrapolated_24h as f64,
        tps_based_24h as f64,
        contracts_estimated as f64,
    ];
    
    let mean = variance.iter().sum::<f64>() / variance.len() as f64;
    let std_dev = {
        let variance_sum: f64 = variance.iter()
            .map(|value| {
                let diff = mean - value;
                diff * diff
            })
            .sum();
        (variance_sum / variance.len() as f64).sqrt()
    };
    
    let coefficient_variation = std_dev / mean;
    let confidence = (1.0 - coefficient_variation.min(1.0)).max(0.0) as f32;
    
    (weighted_estimate, final_tps, confidence)
}

// ===============================
// DÉTECTION D'ANOMALIES
// ===============================

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
    
    // Détecter les pics ou chutes anormales
    for (i, sample) in samples.iter().enumerate() {
        let sample_tps = if sample.sample_period_secs > 0 {
            sample.num_transactions as f64 / sample.sample_period_secs as f64
        } else {
            0.0
        };
        
        // Pic anormal (>150% de la moyenne)
        if sample_tps > avg_tps * 1.5 && avg_tps > 0.0 {
            anomalies.push(format!("Pic TPS détecté: {:.0} (échantillon {})", sample_tps, i));
        }
        
        // Chute anormale (<50% de la moyenne)
        if sample_tps < avg_tps * 0.5 && avg_tps > 0.0 {
            anomalies.push(format!("Chute TPS détectée: {:.0} (échantillon {})", sample_tps, i));
        }
    }
    
    anomalies
}

// ===============================
// ANALYSE DE TENDANCES
// ===============================

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
    
    let change_percent = if first_tps > 0.0 {
        ((second_tps - first_tps) / first_tps) * 100.0
    } else {
        0.0
    };
    
    if change_percent > 10.0 {
        format!("📈 Tendance haussière: +{:.1}% TPS", change_percent)
    } else if change_percent < -10.0 {
        format!("📉 Tendance baissière: {:.1}% TPS", change_percent)
    } else {
        format!("➡️ Tendance stable: {:.1}% TPS", change_percent)
    }
}

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
// UTILITAIRES D'AFFICHAGE
// ===============================

fn format_number(num: u64) -> String {
    num.to_string()
        .chars()
        .rev()
        .collect::<String>()
        .as_bytes()
        .chunks(3)
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .rev()
        .collect::<String>()
}

pub fn display_analysis_summary(analysis: &SolanaAnalysis) {
    println!("\n🚀 ═══ ANALYSE SOLANA 24H - RÉSULTATS FINAUX ═══ 🚀");
    println!("═══════════════════════════════════════════════════════");
    
    println!("📊 DONNÉES DE BASE:");
    println!("   Performance Samples: {} tx sur {:.1}h", 
             format_number(analysis.performance_total_tx), 
             analysis.performance_period_hours);
    println!("   TPS réel observé: {:.0}", analysis.performance_tps);
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
    
    println!("\n⏱️ PERFORMANCE ANALYSE:");
    println!("   Temps de collecte: {}ms", analysis.analysis_time_ms);
    println!("   Méthodes utilisées: {}", analysis.methods_used.join(", "));
    
    println!("═══════════════════════════════════════════════════════");
}