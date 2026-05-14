# thermal-fluids-animator

A small Rust generator for self-contained, browser-ready SVG/HTML lessons for Mechanical PE Thermal and Fluids study.

## MVP lessons

Running the binary writes three standalone HTML files:

- `pipe_head_loss_demo.html` — Darcy-Weisbach head loss in a flowing pipe.
- `pump_curve_demo.html` — pump curve/system curve operating point.
- `heat_exchanger_demo.html` — counterflow heat exchanger effectiveness and energy balance.

Each file contains inline CSS, SVG, and JavaScript, so it can be opened directly in a browser without a server.

## Generate demos

```bash
cargo run --manifest-path thermal-fluids-animator/Cargo.toml -- thermal-fluids-animator/dist
```

Then open any generated file from `thermal-fluids-animator/dist/` in a browser.

## Physics helpers

The Rust library exposes reusable helper functions for common Thermal/Fluids calculations:

- `pipe_area(D) = πD²/4`
- `velocity(Q, A) = Q/A`
- `reynolds_number(ρ, V, D, μ) = ρVD/μ`
- `darcy_weisbach_head_loss(f, L, D, V) = f(L/D)(V²/2g)`
- `pressure_drop(ρ, h_f) = ρgh_f`
- `pump_power(ρ, Q, H, η) = ρgQH/η`
- `heat_transfer_rate(ṁ, cp, ΔT) = ṁcpΔT`
