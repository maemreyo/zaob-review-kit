# MLE Standard — ML / AI Engineer

**Perspective:** Model custodian. Is the ML system correct, fair, reproducible, and production-safe?

References: Google ML Best Practices, EU AI Act (2024), NIST AI RMF, MLflow/W&B conventions.

---

## 1. Data Quality & Leakage Prevention

- [ ] Training, validation, and test sets are strictly separated — absolutely no data leakage
- [ ] Preprocessing (scaling, encoding, imputation) fit **only** on training data, then applied to val/test
- [ ] Dataset version is tracked and referenced in the experiment log
- [ ] Class balance and demographic coverage documented
- [ ] Synthetic / augmented data clearly labelled and does not distort evaluation metrics

## 2. Model Fairness & Bias

- [ ] Fairness metrics defined for the use case (demographic parity, equalised odds, calibration by group)
- [ ] Model performance evaluated across demographic slices, not just overall aggregate
- [ ] Protected attributes not used directly; proxy features audited for discriminatory correlation
- [ ] If bias detected: mitigation technique applied (pre/in/post-processing) and documented
- [ ] Fairness vs. accuracy trade-off explicitly decided and written down

## 3. Reproducibility

- [ ] Experiment tracked: hyperparameters, dataset version, random seeds, environment pinned
- [ ] Model artefact version-controlled and tagged in model registry with training metadata
- [ ] Training run is reproducible with the same seeds (verified by re-running)
- [ ] Baseline comparison documented — is the new model actually better than what's in production?

## 4. Evaluation Correctness

- [ ] Correct metric chosen for the task (accuracy vs. F1 vs. AUC vs. NDCG — depends on class balance and cost of errors)
- [ ] Both optimising metric and satisficing metric defined (e.g., "maximise recall, subject to precision ≥ 0.8")
- [ ] Calibration checked — predicted probabilities reflect actual frequencies
- [ ] Evaluation on held-out golden benchmark dataset, not just the validation set used for tuning

## 5. Model Drift & Monitoring

- [ ] Data drift monitoring defined (PSI, KS test, or equivalent for feature distributions)
- [ ] Concept drift monitoring defined (production performance tracked vs. offline baseline)
- [ ] Retraining trigger thresholds documented (e.g., "retrain when accuracy drops > 5% for 3 consecutive days")
- [ ] Alerting in place for drift events
- [ ] Escalation path clear: who decides to retrain vs. rollback?

## 6. Inference & Serving

- [ ] Inference latency meets the SLO (p95 target defined and measured)
- [ ] Model loaded once at startup, not per-request
- [ ] Input validated at inference time (type, shape, value range)
- [ ] Graceful fallback defined when model is unavailable or returns low-confidence output

## 7. LLM / Generative AI (if applicable)

- [ ] Prompt injection mitigated — untrusted user input not directly interpolated into system prompts
- [ ] Indirect prompt injection defended — external content (emails, web pages) reaching the LLM is sandboxed
- [ ] LLM output validated / sanitised before acting on it (never `eval()` or `exec()` on generated code)
- [ ] Token budget and cost-per-call estimated and approved for production volume
- [ ] PII not sent to third-party LLM APIs without consent and a DPA in place
- [ ] Hallucination risk assessed: where is the human-in-the-loop for consequential outputs?

## 8. EU AI Act Classification (if applicable)

- [ ] System risk tier identified: Unacceptable / High-risk / Limited / Minimal
- [ ] High-risk system (Annex III): conformity assessment pathway identified
- [ ] Transparency obligation met: users know they are interacting with AI (Art. 52)
- [ ] Human oversight mechanism defined for consequential AI decisions

## Output Format

```
[BLOCKER] The StandardScaler is fit on the combined train+validation+test split
(line 34 of preprocessing.py). This causes data leakage — the model has seen
the statistical properties of the test set during training. Fit only on X_train.

[MAJOR] The model is evaluated only on overall accuracy (92%). The dataset is
87% majority class — a dummy classifier that always predicts the majority class
would score 87%. Add F1, precision, recall, and confusion matrix. Evaluate
separately on the minority class.

[QUESTION] What is the retraining schedule? If the recommendation model is
serving live users but only retrained monthly, a concept drift event could
degrade recommendations for up to 30 days before it is caught.
```
