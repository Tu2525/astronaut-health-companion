# MSL/RAD daily dose rate, January 2015 (Mars surface, Curiosity)

Reference data only, to shape the simulated demo. Not crew data. Do not label simulated values as NASA data.

- **Dataset:** MSL/RAD Level 2 observational radiation data (CDAWeb id `MSL_RAD_OBS-L2`), PI D. Hassler, SwRI-Boulder
- **URL:** https://cdaweb.gsfc.nasa.gov/WS/cdasr/1/dataviews/sp_phys/datasets/MSL_RAD_OBS-L2/data/20150101T000000Z,20150131T000000Z/DOSIMETRY_TOTAL_f?format=csv  (CDAWeb REST web service; variable DOSIMETRY_TOTAL_f, good-quality only)
- **DOI:** 10.48322/20qb-4e17 (SPASE: spase://NASA/NumericalData/MarsScienceLaboratory/RAD/Observation/Level2)
- **Terms:** NASA open data. CDAWeb RULES_OF_USE: "archived as fully citable data; consult the RAD team for processing and usage". ACKNOWLEDGEMENT: acknowledge the RAD Principal Investigator Dr. Hassler of Southwest Research Institute Boulder. No explicit license text was returned; cite the DOI.
- **Retrieved:** 2026-11-14

## Columns (msl_rad_daily_2015-01.csv)
| column | meaning | unit |
|---|---|---|
| date_utc | UTC day | date |
| n_obs | number of 16-min observations averaged | count |
| total_dose_si_uGy_per_day | dose rate in silicon detector B (includes MMRTG background) | uGy/day |
| rtg_corrected_dose_si_uGy_per_day | same, MMRTG contribution subtracted (key column) | uGy/day |
| total_dose_tissue_plastic_uGy_per_day | dose rate in tissue-equivalent plastic detector E | uGy/day |

## Trimming
Raw download was 2048 rows (about 16-min cadence, 1-31 Jan 2015). Averaged to one row per UTC day. Rows with non-positive or fill (>=1e5) values dropped, so only 25 of 31 days appear (gaps = no good-quality data).
Mean of per-observation values; no thresholds or status computed.

## Summary of rtg_corrected_dose_si_uGy_per_day (25 days)
- min 126.2
- mean 143.8
- max 150.4
(about 0.13-0.15 mGy/day on the Mars surface; uGy/day, not mSv/day, no quality-factor applied)
