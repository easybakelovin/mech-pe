//! Reusable physics and lesson HTML generation for Thermal/Fluids PE animations.

use std::f64::consts::PI;

/// Standard gravitational acceleration in m/s².
pub const G: f64 = 9.80665;
/// Approximate density of water at room temperature in kg/m³.
pub const WATER_RHO: f64 = 998.0;
/// Approximate dynamic viscosity of water at room temperature in Pa·s.
pub const WATER_MU: f64 = 0.001_002;
/// Approximate specific heat of liquid water in J/(kg·K).
pub const WATER_CP: f64 = 4_186.0;

/// Cross-sectional area for a circular pipe, `A = πD²/4`.
pub fn pipe_area(diameter_m: f64) -> f64 {
    PI * diameter_m.powi(2) / 4.0
}

/// Average velocity from volumetric flow and cross-sectional area, `V = Q/A`.
pub fn velocity(flow_m3_s: f64, area_m2: f64) -> f64 {
    flow_m3_s / area_m2
}

/// Reynolds number, `Re = ρVD/μ`.
pub fn reynolds_number(rho_kg_m3: f64, velocity_m_s: f64, diameter_m: f64, mu_pa_s: f64) -> f64 {
    rho_kg_m3 * velocity_m_s * diameter_m / mu_pa_s
}

/// Darcy-Weisbach head loss, `h_f = f(L/D)(V²/2g)`.
pub fn darcy_weisbach_head_loss(
    friction_factor: f64,
    length_m: f64,
    diameter_m: f64,
    velocity_m_s: f64,
) -> f64 {
    friction_factor * (length_m / diameter_m) * (velocity_m_s.powi(2) / (2.0 * G))
}

/// Pressure drop from head loss, `Δp = ρgh_f`.
pub fn pressure_drop(rho_kg_m3: f64, head_loss_m: f64) -> f64 {
    rho_kg_m3 * G * head_loss_m
}

/// Hydraulic pump power, `P = ρgQH/η`.
pub fn pump_power(rho_kg_m3: f64, flow_m3_s: f64, head_m: f64, efficiency: f64) -> f64 {
    rho_kg_m3 * G * flow_m3_s * head_m / efficiency
}

/// Sensible heat transfer rate, `q̇ = ṁ cp ΔT`.
pub fn heat_transfer_rate(m_dot_kg_s: f64, cp_j_kg_k: f64, delta_t_k: f64) -> f64 {
    m_dot_kg_s * cp_j_kg_k * delta_t_k
}

/// Returns every MVP lesson as `(file_name, html)` pairs.
pub fn render_lessons() -> Vec<(String, String)> {
    vec![
        ("pipe_head_loss_demo.html".into(), pipe_head_loss_demo()),
        ("pump_curve_demo.html".into(), pump_curve_demo()),
        ("heat_exchanger_demo.html".into(), heat_exchanger_demo()),
    ]
}

const SHARED_STYLE: &str = r#"
:root { color-scheme: light; --ink:#18212f; --muted:#64748b; --blue:#2563eb; }
* { box-sizing: border-box; }
body { margin:0; font-family: Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; color:var(--ink); background: radial-gradient(circle at 10% 0%, #e0f2fe 0, #f8fafc 34%, #eef2ff 100%); }
main { max-width: 1180px; margin: 0 auto; padding: 32px 20px 44px; }
.hero { display:flex; justify-content:space-between; gap:24px; align-items:flex-end; margin-bottom:22px; }
h1 { margin:0 0 8px; font-size: clamp(2rem, 4vw, 3.4rem); line-height:1; letter-spacing:-0.04em; }
p { color:var(--muted); line-height:1.55; margin:0; }
.lesson-grid { display:grid; grid-template-columns: 360px minmax(0, 1fr); gap:20px; align-items:start; }
.panel, .stage { background:rgba(255,255,255,.82); border:1px solid rgba(148,163,184,.35); box-shadow: 0 20px 60px rgba(15,23,42,.10); border-radius:24px; padding:20px; backdrop-filter: blur(14px); }
.control { margin: 0 0 18px; }
.control label { display:flex; justify-content:space-between; gap:12px; font-weight:700; font-size:.94rem; margin-bottom:8px; }
.control output { color:var(--blue); font-variant-numeric: tabular-nums; }
input[type=range] { width:100%; accent-color: var(--blue); }
.kpis { display:grid; grid-template-columns: 1fr 1fr; gap:10px; margin-top:18px; }
.kpi { padding:14px; border-radius:18px; background:#f8fafc; border:1px solid #e2e8f0; }
.kpi strong { display:block; font-size:.78rem; text-transform:uppercase; letter-spacing:.08em; color:var(--muted); }
.kpi span { display:block; margin-top:6px; font-size:1.35rem; font-weight:800; font-variant-numeric: tabular-nums; }
.note { margin-top:16px; padding:14px; border-radius:18px; background:#eff6ff; border:1px solid #bfdbfe; font-size:.95rem; color:#334155; }
svg { width:100%; height:auto; display:block; overflow:visible; }
.equation { font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace; color:#334155; }
.flow-dot { animation: dash 1.5s linear infinite; }
@keyframes dash { from { stroke-dashoffset: 80; } to { stroke-dashoffset: 0; } }
@media (max-width: 900px) { .lesson-grid { grid-template-columns: 1fr; } .hero { display:block; } }
"#;

fn page(title: &str, subtitle: &str, controls: &str, stage: &str, script: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<style>{SHARED_STYLE}</style>
</head>
<body>
<main>
  <section class="hero">
    <div><h1>{title}</h1><p>{subtitle}</p></div>
    <p class="equation">Thermal &amp; Fluids PE interactive SVG lesson</p>
  </section>
  <section class="lesson-grid">
    <aside class="panel">{controls}</aside>
    <article class="stage">{stage}</article>
  </section>
</main>
<script>{script}</script>
</body>
</html>"#
    )
}

fn pipe_head_loss_demo() -> String {
    page(
        "Pipe Head Loss",
        "Adjust pipe and flow inputs to connect Darcy-Weisbach math to velocity, turbulence, and pressure drop.",
        r#"
<div class="control"><label>Flow rate Q <output id="qOut"></output></label><input id="q" type="range" min="0.01" max="0.20" step="0.005" value="0.08"></div>
<div class="control"><label>Pipe diameter D <output id="dOut"></output></label><input id="d" type="range" min="0.05" max="0.35" step="0.005" value="0.15"></div>
<div class="control"><label>Pipe length L <output id="lOut"></output></label><input id="l" type="range" min="10" max="200" step="5" value="80"></div>
<div class="control"><label>Friction factor f <output id="fOut"></output></label><input id="f" type="range" min="0.008" max="0.06" step="0.001" value="0.025"></div>
<div class="kpis"><div class="kpi"><strong>Velocity</strong><span id="vKpi"></span></div><div class="kpi"><strong>Reynolds No.</strong><span id="reKpi"></span></div><div class="kpi"><strong>Head loss</strong><span id="hKpi"></span></div><div class="kpi"><strong>Δp</strong><span id="dpKpi"></span></div></div>
<p class="note"><span class="equation">h_f = f(L/D)(V²/2g)</span>. Higher flow and smaller diameters rapidly increase velocity head.</p>
"#,
        r##"
<svg viewBox="0 0 780 480" role="img" aria-label="Water pipe with animated flow and pressure drop gauges">
  <defs><linearGradient id="pipeGrad" x1="0" x2="1"><stop offset="0" stop-color="#38bdf8"/><stop id="pipeHot" offset="1" stop-color="#2563eb"/></linearGradient><marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#0f172a"/></marker></defs>
  <rect x="56" y="176" width="668" height="128" rx="64" fill="url(#pipeGrad)" stroke="#0f172a" stroke-width="4"/><rect x="74" y="199" width="632" height="82" rx="41" fill="#e0f2fe" opacity=".45"/>
  <path id="flowPath" d="M120 240 H660" stroke="#075985" stroke-width="18" stroke-linecap="round" stroke-dasharray="34 46" class="flow-dot" opacity=".78"/>
  <path d="M112 122 H300" stroke="#0f172a" stroke-width="5" marker-end="url(#arrow)"/><text x="116" y="104" font-size="22" font-weight="800">flow accelerates</text>
  <g transform="translate(96 340)"><circle r="54" fill="#fff" stroke="#0f172a" stroke-width="4"/><path id="p1Needle" d="M0 0 L0 -38" stroke="#16a34a" stroke-width="5" stroke-linecap="round"/><text x="-34" y="84" font-size="18" font-weight="800">upstream</text></g>
  <g transform="translate(684 340)"><circle r="54" fill="#fff" stroke="#0f172a" stroke-width="4"/><path id="p2Needle" d="M0 0 L0 -38" stroke="#ef4444" stroke-width="5" stroke-linecap="round"/><text x="-48" y="84" font-size="18" font-weight="800">downstream</text></g>
  <path id="dropBar" d="M170 370 H610" stroke="#f97316" stroke-width="16" stroke-linecap="round" opacity=".25"/><text id="regime" x="390" y="398" text-anchor="middle" font-size="24" font-weight="900" fill="#0f172a"></text>
</svg>
"##,
        r#"
const g=9.80665, rho=998, mu=0.001002, $=id=>document.getElementById(id);
function area(D){return Math.PI*D*D/4} function velocity(Q,A){return Q/A} function re(rho,V,D,mu){return rho*V*D/mu} function headLoss(f,L,D,V){return f*(L/D)*(V*V/(2*g))} function deltaP(rho,h){return rho*g*h}
function fmt(n,d=2){return Number(n).toLocaleString(undefined,{maximumFractionDigits:d,minimumFractionDigits:d})}
function update(){const Q=+$('q').value,D=+$('d').value,L=+$('l').value,f=+$('f').value,V=velocity(Q,area(D)),Re=re(rho,V,D,mu),h=headLoss(f,L,D,V),dp=deltaP(rho,h);$('qOut').value=fmt(Q,3)+' m³/s';$('dOut').value=fmt(D,3)+' m';$('lOut').value=fmt(L,0)+' m';$('fOut').value=fmt(f,3);$('vKpi').textContent=fmt(V)+' m/s';$('reKpi').textContent=fmt(Re,0);$('hKpi').textContent=fmt(h)+' m';$('dpKpi').textContent=fmt(dp/1000)+' kPa';const intensity=Math.min(1,h/80);$('pipeHot').setAttribute('stop-color',`rgb(${37+Math.round(210*intensity)}, ${99-Math.round(40*intensity)}, ${235-Math.round(190*intensity)})`);$('flowPath').style.animationDuration=Math.max(.25,2.2-V/4)+'s';$('flowPath').setAttribute('stroke-width',String(10+Math.min(18,V*3)));$('p1Needle').setAttribute('transform','rotate(-45)');$('p2Needle').setAttribute('transform',`rotate(${45+intensity*115})`);$('dropBar').setAttribute('opacity',.18+.65*intensity);$('regime').textContent=Re<2300?'Laminar range':(Re<4000?'Transitional range':'Turbulent range')}
document.querySelectorAll('input').forEach(i=>i.addEventListener('input',update)); update();
"#,
    )
}

fn pump_curve_demo() -> String {
    page(
        "Pump Curve Operating Point",
        "Move pump speed and system resistance to see where the pump curve intersects the piping system curve.",
        r#"
<div class="control"><label>Pump speed <output id="speedOut"></output></label><input id="speed" type="range" min="60" max="120" step="1" value="100"></div>
<div class="control"><label>System resistance K <output id="kOut"></output></label><input id="k" type="range" min="400" max="2600" step="25" value="1200"></div>
<div class="kpis"><div class="kpi"><strong>Flow Q</strong><span id="qKpi"></span></div><div class="kpi"><strong>Head H</strong><span id="hKpi"></span></div><div class="kpi"><strong>Pump power</strong><span id="pKpi"></span></div><div class="kpi"><strong>Efficiency used</strong><span>72%</span></div></div>
<p class="note">The operating point is the simultaneous solution of <span class="equation">H_pump(Q)</span> and <span class="equation">H_system(Q)</span>.</p>
"#,
        r##"
<svg viewBox="0 0 820 520" role="img" aria-label="Pump curve and system curve with animated operating point">
  <defs><marker id="arrow2" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#0f172a"/></marker></defs>
  <g transform="translate(50 52)"><circle cx="90" cy="105" r="56" fill="#dbeafe" stroke="#0f172a" stroke-width="4"/><path id="impeller" d="M90 105 C112 78 125 96 98 106 C77 132 60 114 84 104" fill="#2563eb" opacity=".85"/><path d="M145 105 H330" stroke="#38bdf8" stroke-width="34" stroke-linecap="round"/><path id="pumpFlow" d="M160 105 H320" stroke="#075985" stroke-width="8" stroke-dasharray="16 22" class="flow-dot" marker-end="url(#arrow2)"/><text x="18" y="195" font-size="20" font-weight="900">pump + pipe</text></g>
  <g transform="translate(380 42)"><rect x="0" y="0" width="390" height="390" rx="22" fill="#fff" stroke="#cbd5e1"/><line x1="54" y1="330" x2="350" y2="330" stroke="#0f172a" stroke-width="3"/><line x1="54" y1="330" x2="54" y2="34" stroke="#0f172a" stroke-width="3"/><text x="202" y="372" text-anchor="middle" font-size="16" font-weight="800">flow rate Q (m³/s)</text><text x="16" y="190" transform="rotate(-90 16 190)" text-anchor="middle" font-size="16" font-weight="800">head H (m)</text><path id="pumpCurve" fill="none" stroke="#2563eb" stroke-width="5"/><path id="sysCurve" fill="none" stroke="#f97316" stroke-width="5"/><circle id="opPoint" r="9" fill="#16a34a" stroke="#052e16" stroke-width="3"/><text id="opLabel" x="0" y="0" font-size="16" font-weight="900" fill="#166534"></text><text x="245" y="68" fill="#2563eb" font-size="16" font-weight="900">pump curve</text><text x="214" y="306" fill="#f97316" font-size="16" font-weight="900">system curve</text></g>
</svg>
"##,
        r#"
const rho=998,g=9.80665,eta=.72,$=id=>document.getElementById(id),qMax=.24,hMax=95;
function pumpHead(Q,s){const n=s/100;return Math.max(0,72*n*n-1050*Q*Q)} function sysHead(Q,k){return 6+k*Q*Q} function pumpPower(rho,Q,H,eta){return rho*g*Q*H/eta}
function x(Q){return 434+Q/qMax*296} function y(H){return 372-H/hMax*296} function pathFor(fn){let d='';for(let i=0;i<=80;i++){const Q=qMax*i/80,H=fn(Q);d+=(i?'L':'M')+x(Q)+' '+y(H)+' '}return d} function solve(s,k){let lo=0,hi=qMax;for(let i=0;i<70;i++){let mid=(lo+hi)/2;if(pumpHead(mid,s)>sysHead(mid,k))lo=mid;else hi=mid}const Q=(lo+hi)/2;return {Q,H:sysHead(Q,k)}} function fmt(n,d=2){return Number(n).toLocaleString(undefined,{maximumFractionDigits:d,minimumFractionDigits:d})}
function update(){const s=+$('speed').value,k=+$('k').value,op=solve(s,k);$('speedOut').value=s+'%';$('kOut').value=fmt(k,0);$('qKpi').textContent=fmt(op.Q,3)+' m³/s';$('hKpi').textContent=fmt(op.H)+' m';$('pKpi').textContent=fmt(pumpPower(rho,op.Q,op.H,eta)/1000)+' kW';$('pumpCurve').setAttribute('d',pathFor(Q=>pumpHead(Q,s)));$('sysCurve').setAttribute('d',pathFor(Q=>sysHead(Q,k)));$('opPoint').setAttribute('cx',x(op.Q));$('opPoint').setAttribute('cy',y(op.H));$('opLabel').setAttribute('x',x(op.Q)+14);$('opLabel').setAttribute('y',y(op.H)-12);$('opLabel').textContent='operating point';$('pumpFlow').style.animationDuration=Math.max(.25,1.8-op.Q*5)+'s';$('impeller').setAttribute('transform',`rotate(${(Date.now()/20*s)%360} 90 105)`)}
document.querySelectorAll('input').forEach(i=>i.addEventListener('input',update)); setInterval(update,80); update();
"#,
    )
}

fn heat_exchanger_demo() -> String {
    page(
        "Counterflow Heat Exchanger",
        "Explore energy balance and effectiveness by changing inlet temperatures, mass flow rates, and exchanger effectiveness.",
        r#"
<div class="control"><label>Hot inlet temperature <output id="thOut"></output></label><input id="th" type="range" min="60" max="180" step="1" value="130"></div>
<div class="control"><label>Cold inlet temperature <output id="tcOut"></output></label><input id="tc" type="range" min="5" max="55" step="1" value="25"></div>
<div class="control"><label>Hot mass flow ṁh <output id="mhOut"></output></label><input id="mh" type="range" min="0.2" max="5" step="0.1" value="1.5"></div>
<div class="control"><label>Cold mass flow ṁc <output id="mcOut"></output></label><input id="mc" type="range" min="0.2" max="5" step="0.1" value="2.0"></div>
<div class="control"><label>Effectiveness ε <output id="epsOut"></output></label><input id="eps" type="range" min="0.10" max="0.95" step="0.01" value="0.70"></div>
<div class="kpis"><div class="kpi"><strong>q̇</strong><span id="qKpi"></span></div><div class="kpi"><strong>Hot outlet</strong><span id="thoKpi"></span></div><div class="kpi"><strong>Cold outlet</strong><span id="tcoKpi"></span></div><div class="kpi"><strong>Cmin</strong><span id="cminKpi"></span></div></div>
<p class="note"><span class="equation">q̇ = ε Cmin(Th,in − Tc,in)</span>, then outlet temperatures follow from each stream energy balance.</p>
"#,
        r##"
<svg viewBox="0 0 820 470" role="img" aria-label="Counterflow heat exchanger with hot and cold stream arrows">
  <defs><marker id="hotArrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="8" markerHeight="8" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#ef4444"/></marker><marker id="coldArrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="8" markerHeight="8" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="#2563eb"/></marker></defs>
  <rect x="165" y="105" width="490" height="210" rx="34" fill="#f8fafc" stroke="#0f172a" stroke-width="4"/><path id="hotTube" d="M690 158 H130" stroke="#ef4444" stroke-width="44" stroke-linecap="round" marker-end="url(#hotArrow)"/><path id="coldTube" d="M130 262 H690" stroke="#2563eb" stroke-width="44" stroke-linecap="round" marker-end="url(#coldArrow)"/><path id="heatArrow" d="M410 175 V242" stroke="#f97316" stroke-width="14" stroke-linecap="round" marker-end="url(#hotArrow)" opacity=".7"/><path d="M640 80 C570 128 504 94 442 132 C365 180 286 126 192 154" fill="none" stroke="#f97316" stroke-width="4" stroke-dasharray="8 12" opacity=".7"/>
  <text id="thi" x="672" y="115" text-anchor="middle" font-size="20" font-weight="900" fill="#991b1b"></text><text id="tho" x="134" y="115" text-anchor="middle" font-size="20" font-weight="900" fill="#991b1b"></text><text id="tci" x="130" y="340" text-anchor="middle" font-size="20" font-weight="900" fill="#1e3a8a"></text><text id="tco" x="690" y="340" text-anchor="middle" font-size="20" font-weight="900" fill="#1e3a8a"></text><text x="410" y="390" text-anchor="middle" font-size="24" font-weight="900">counterflow: hottest hot meets hottest cold</text><text id="qLabel" x="430" y="222" font-size="20" font-weight="900" fill="#9a3412"></text>
</svg>
"##,
        r#"
const cp=4186,$=id=>document.getElementById(id); function qDot(m,cp,dT){return m*cp*dT} function fmt(n,d=1){return Number(n).toLocaleString(undefined,{maximumFractionDigits:d,minimumFractionDigits:d})}
function update(){const Thi=+$('th').value,Tci=+$('tc').value,mh=+$('mh').value,mc=+$('mc').value,eps=+$('eps').value,Ch=mh*cp,Cc=mc*cp,Cmin=Math.min(Ch,Cc),q=eps*Cmin*Math.max(0,Thi-Tci),Tho=Thi-q/Ch,Tco=Tci+q/Cc;$('thOut').value=fmt(Thi,0)+' °C';$('tcOut').value=fmt(Tci,0)+' °C';$('mhOut').value=fmt(mh,1)+' kg/s';$('mcOut').value=fmt(mc,1)+' kg/s';$('epsOut').value=fmt(eps,2);$('qKpi').textContent=fmt(q/1000,1)+' kW';$('thoKpi').textContent=fmt(Tho,1)+' °C';$('tcoKpi').textContent=fmt(Tco,1)+' °C';$('cminKpi').textContent=fmt(Cmin/1000,2)+' kW/K';$('thi').textContent='Th,in '+fmt(Thi,0)+'°C';$('tho').textContent='Th,out '+fmt(Tho,1)+'°C';$('tci').textContent='Tc,in '+fmt(Tci,0)+'°C';$('tco').textContent='Tc,out '+fmt(Tco,1)+'°C';$('qLabel').textContent=fmt(q/1000,0)+' kW';const heat=Math.min(1,q/500000);$('heatArrow').setAttribute('stroke-width',8+18*heat);$('heatArrow').setAttribute('opacity',.35+.6*heat);$('hotTube').setAttribute('opacity',.55+.45*(Thi/180));$('coldTube').setAttribute('opacity',.55+.45*(1-Tci/80))}
document.querySelectorAll('input').forEach(i=>i.addEventListener('input',update)); update();
"#,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipe_physics_matches_known_values() {
        let area = pipe_area(0.1);
        let v = velocity(0.01, area);
        let re = reynolds_number(WATER_RHO, v, 0.1, WATER_MU);
        let h = darcy_weisbach_head_loss(0.02, 50.0, 0.1, v);
        assert!((area - 0.007_853_981_6).abs() < 1e-9);
        assert!((v - 1.273_239_545).abs() < 1e-9);
        assert!((re - 126_815.0).abs() < 1.0);
        assert!((h - 0.826_5).abs() < 0.001);
    }

    #[test]
    fn pressure_power_and_heat_helpers_are_consistent() {
        assert!((pressure_drop(1000.0, 10.0) - 98_066.5).abs() < 0.1);
        assert!((pump_power(1000.0, 0.05, 20.0, 0.8) - 12_258.3125).abs() < 0.001);
        assert!((heat_transfer_rate(2.0, 4186.0, 10.0) - 83_720.0).abs() < 0.001);
    }
}
