//! Inference subsystem — on-device AI via Candle
//!
//! Future: quantized model loading, sensor-to-decision pipeline,
//! inference latency tracking, model hot-swap.

/// f7=inference_status — report inference subsystem state
pub fn f7() -> &'static str {
    "no model loaded"
}
