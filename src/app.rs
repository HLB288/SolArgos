use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/solana-dashboard.css"/>
        <Title text="Solana Analytics Dashboard"/>
        <Meta name="description" content="Dashboard d'analyse temps réel de la blockchain Solana"/>
        
        <Router>
            <main class="min-h-screen bg-gray-900 text-white">
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/dashboard" view=Dashboard/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900">
            <div class="text-center p-8 max-w-4xl mx-auto">
                <h1 class="text-6xl font-bold mb-6 bg-gradient-to-r from-purple-400 to-pink-400 bg-clip-text text-transparent">
                    "Solana Analytics"
                </h1>
                <p class="text-xl text-gray-300 mb-8">
                    "Dashboard d'analyse temps réel de la blockchain Solana"
                </p>
                
                <A href="/dashboard" 
                   class="inline-block bg-gradient-to-r from-purple-600 to-pink-600 text-white px-8 py-4 rounded-lg text-lg font-semibold hover:from-purple-700 hover:to-pink-700 transition-all duration-300">
                    "🚀 Accéder au Dashboard"
                </A>
            </div>
        </div>
    }
}

#[component]
fn Dashboard() -> impl IntoView {
    let (loading, set_loading) = create_signal(true);

    create_effect(move |_| {
        set_loading.set(true);
        
        #[cfg(target_arch = "wasm32")]
        {
            let timeout_id = gloo::timers::callback::Timeout::new(2000, move || {
                set_loading.set(false);
            });
            timeout_id.forget();
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            set_loading.set(false);
        }
    });

    view! {
        <div class="min-h-screen bg-gray-900 p-6">
            <div class="max-w-7xl mx-auto">
                <div class="mb-8">
                    <A href="/" class="text-gray-400 hover:text-white mb-4 inline-block">
                        "← Retour à l'accueil"
                    </A>
                    <h1 class="text-4xl font-bold text-center text-white">
                        "📊 Dashboard Solana"
                    </h1>
                </div>
                
                {move || {
                    if loading.get() {
                        view! {
                            <div class="text-center py-16">
                                <div class="animate-spin rounded-full h-16 w-16 border-b-2 border-purple-500 mx-auto mb-4"></div>
                                <span class="text-white text-lg">"Chargement..."</span>
                            </div>
                        }
                    } else {
                        view! {
                            <div class="text-center p-8 bg-gray-800 rounded-lg border border-green-600">
                                <h2 class="text-2xl text-green-400 mb-4">"✅ Dashboard prêt !"</h2>
                                <p class="text-gray-300">"🎯 Prêt pour l'intégration Solana !"</p>
                            </div>
                        }
                    }
                }}
            </div>
        </div>
    }
}
#[component]
fn MetricCard(
    title: &'static str,
    value: String,
    icon: &'static str,
    color: &'static str,
    subtitle: String,
) -> impl IntoView {
    let color_classes = match color {
        "purple" => "border-purple-600 bg-purple-900/20",
        "blue" => "border-blue-600 bg-blue-900/20", 
        "green" => "border-green-600 bg-green-900/20",
        "pink" => "border-pink-600 bg-pink-900/20",
        _ => "border-gray-600 bg-gray-900/20",
    };

    let icon_color = match color {
        "purple" => "text-purple-400",
        "blue" => "text-blue-400", 
        "green" => "text-green-400",
        "pink" => "text-pink-400",
        _ => "text-gray-400",
    };

    view! {
        <div class=format!("bg-gray-800 rounded-lg p-6 border-2 {} hover:scale-105 transition-all duration-300", color_classes)>
            <div class="flex items-center justify-between mb-2">
                <span class="text-gray-400 text-sm font-medium">{title}</span>
                <span class=format!("text-2xl {}", icon_color)>{icon}</span>
            </div>
            <div class="text-2xl font-bold text-white mb-1">{value}</div>
            <div class="text-xs text-gray-500">{subtitle}</div>
        </div>
    }
}

// Fonction utilitaire pour formater les grands nombres
fn format_large_number(num: u64) -> String {
    if num >= 1_000_000_000 {
        format!("{:.1}B", num as f64 / 1_000_000_000.0)
    } else if num >= 1_000_000 {
        format!("{:.1}M", num as f64 / 1_000_000.0)
    } else if num >= 1_000 {
        format!("{:.1}K", num as f64 / 1_000.0)
    } else {
        num.to_string()
    }
}