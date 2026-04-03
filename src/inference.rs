//! Inference subsystem — on-device AI via Candle
//!
//! T6=InferenceEngine trait, T7=Prediction result type.
//! Future: quantized model loading, sensor-to-decision pipeline,
//! inference latency tracking, model hot-swap.

/// T7=Prediction — inference result
#[derive(Debug, Clone)]
pub struct T7 {
    pub label: String,
    pub confidence: f32,
    pub latency_ms: u64,
}

/// T6=InferenceEngine — trait for on-device model inference
pub trait T6 {
    /// Load model from bytes or path. Returns model name on success.
    fn load_model(&mut self, name: &str, data: &[u8]) -> Result<(), String>;
    /// Run inference on input features. Returns ranked predictions.
    fn predict(&self, input: &[f32]) -> Result<Vec<T7>, String>;
    /// Current engine status
    fn status(&self) -> &str;
    /// Name of loaded model, if any
    fn model_name(&self) -> Option<&str>;
}

/// T11=MockEngine — test inference engine with canned predictions
pub struct T11 {
    model: Option<String>,
    predictions: Vec<(String, f32)>,
}

impl Default for T11 {
    fn default() -> Self {
        Self::new()
    }
}

impl T11 {
    pub fn new() -> Self {
        Self {
            model: None,
            predictions: Vec::new(),
        }
    }

    /// Set what predictions this mock returns
    pub fn set_predictions(&mut self, preds: Vec<(String, f32)>) {
        self.predictions = preds;
    }
}

impl T6 for T11 {
    fn load_model(&mut self, name: &str, data: &[u8]) -> Result<(), String> {
        if data.is_empty() {
            return Err("empty model data".into());
        }
        self.model = Some(name.to_string());
        Ok(())
    }

    fn predict(&self, input: &[f32]) -> Result<Vec<T7>, String> {
        if self.model.is_none() {
            return Err("no model loaded".into());
        }
        if input.is_empty() {
            return Err("empty input".into());
        }
        Ok(self
            .predictions
            .iter()
            .map(|(label, conf)| T7 {
                label: label.clone(),
                confidence: *conf,
                latency_ms: 1,
            })
            .collect())
    }

    fn status(&self) -> &str {
        match &self.model {
            Some(_) => "ready (mock)",
            None => "no model loaded",
        }
    }

    fn model_name(&self) -> Option<&str> {
        self.model.as_deref()
    }
}

/// f7=inference_status — report inference subsystem state (legacy compat)
pub fn f7() -> &'static str {
    "no model loaded"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_engine_lifecycle() {
        let mut eng = T11::new();
        assert_eq!(eng.status(), "no model loaded");
        assert!(eng.model_name().is_none());

        eng.load_model("test-3b", b"fake weights").unwrap();
        assert_eq!(eng.status(), "ready (mock)");
        assert_eq!(eng.model_name(), Some("test-3b"));
    }

    #[test]
    fn mock_engine_empty_model() {
        let mut eng = T11::new();
        assert!(eng.load_model("bad", b"").is_err());
    }

    #[test]
    fn mock_engine_predict() {
        let mut eng = T11::new();
        eng.load_model("test", b"data").unwrap();
        eng.set_predictions(vec![
            ("alert".into(), 0.92),
            ("normal".into(), 0.08),
        ]);

        let preds = eng.predict(&[1.0, 2.0, 3.0]).unwrap();
        assert_eq!(preds.len(), 2);
        assert_eq!(preds[0].label, "alert");
        assert!((preds[0].confidence - 0.92).abs() < f32::EPSILON);
    }

    #[test]
    fn mock_engine_predict_no_model() {
        let eng = T11::new();
        assert!(eng.predict(&[1.0]).is_err());
    }

    #[test]
    fn mock_engine_predict_empty_input() {
        let mut eng = T11::new();
        eng.load_model("test", b"data").unwrap();
        assert!(eng.predict(&[]).is_err());
    }

    #[test]
    fn legacy_status() {
        assert_eq!(f7(), "no model loaded");
    }
}
