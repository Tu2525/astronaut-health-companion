# Range sources (ENGINE-6)

Status meanings: `sourced` = numbers taken from a NASA document; `adapted` = our range, tied to a NASA document for context (for the vitals the numeric band is a general adult clinical range, not a NASA number); `placeholder` = our own number, NASA context only. Retrieved 2026-11-14. No NASA document found states a numeric normal range for the vitals, so none is `sourced`.

| Indicator | Status | NASA source | What it supports |
|---|---|---|---|
| heart_rate, bp_systolic, bp_diastolic | adapted | [HRP Cardiovascular Evidence Report 2022](https://humanresearchroadmap.nasa.gov/Evidence/reports/Cardio%20Risk%20EB%202022%20FINAL_4-3-2023.pdf) | In-flight HR and BP are lower than pre-flight (the `expected_change` text). No numeric limits. |
| body_temp | adapted | [HSRB Food and Nutrition Risk Record Rev D, 2025](https://www.nasa.gov/wp-content/uploads/2025/08/food-risk-record-rev-d-web-final.pdf) | Core temperature higher in flight. No degC limits. |
| blood_glucose | adapted | Same Cardiovascular report | Insulin resistance rises in flight; glucose alone is not a clean signal. |
| spo2 | adapted | [OCHMO-TB-004 Carbon Dioxide Rev D](https://www.nasa.gov/wp-content/uploads/2023/12/ochmo-tb-004-carbon-dioxide.pdf) | Cabin CO2 hazard (NASA-STD-3001 V2 6004, 3 mmHg 1-h limit). No SpO2 range. |
| sleep_hours | adapted | [HRP Sleep Loss Evidence Report 2016](https://humanresearchroadmap.nasa.gov/evidence/reports/sleep.pdf) | Crews average about 6 h in flight; report cites 7-9 h adult guidance. Caution 4 h and 11 h are ours. |
| mood, stress | placeholder | none found | Self-report 1-5 scale; no NASA numeric range. |
| radiation_dose | adapted | [NTRS 20240009831](https://ntrs.nasa.gov/api/citations/20240009831/downloads/NAS%20BPS%20Simonsen%20v4%20strives.pdf); NASA-STD-3001 Vol 1 [V1 4030]/[V1 4031] via [OCHMO-TB-020](https://www.nasa.gov/wp-content/uploads/2023/03/radiation-protection-technical-brief-ochmo.pdf) | Measured: ISS about 0.6, lunar surface about 0.9, Mars transit about 1.7 mSv/day. Official limits are cumulative (600 mSv career, 250 mSv per SPE), not daily. Our 3 and 10 mSv/day bands are ours. |

Cross-cutting: NASA-STD-3001 Vol 1 Rev C [V1 3004] requires vital signs to be monitored and trended in flight.

## Dataset

`crates/core/data/msl_rad_daily_2015-01.csv`: NASA MSL/RAD daily dose rate, Mars surface, Jan 2015, 25 days, about 0.14 mGy/day (uGy, not mSv). Source, DOI and terms in `crates/core/data/README.md`. Used only to check the order of magnitude of the simulated dosimeter; it is surface data, not transit, and not crew data.

## Not verified

- The Mars-transit 1.7 and surface 0.64 mSv/day pairing was read from slide text; check the slide before quoting.
- No Orion measured mSv/day found; Artemis I modelled mean is about 0.87 mSv/day (22.3 mSv over 25.5 days).
