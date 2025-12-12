pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
use wasm_bindgen::prelude::*;

// Quantinuum-Style: Kleiner, deterministischer Speicher
static mut NOISE_BUDGET: f64 = 0.0;

#[wasm_bindgen]
pub fn init_decoder(start_noise: f64) {
    unsafe { NOISE_BUDGET = start_noise; }
}

// Die Entscheidungskerne (Void/Int Call Pattern)
#[wasm_bindgen]
pub fn update_noise_metric(observed_variance: f64) {
    // Simuliert eine Kalman-Filter Anpassung (leichtgewichtig)
    unsafe { 
        NOISE_BUDGET = (NOISE_BUDGET * 0.8) + (observed_variance * 0.2); 
    }
}

#[wasm_bindgen]
pub fn decide_action() -> i32 {
    unsafe {
        // Policy: Variance > 2^-60 (ca 1.0e-18)
        let threshold = 1.0e-18;
        
        if NOISE_BUDGET > threshold {
            return 1; // ACTION: TRIGGER_BOOTSTRAP
        } else {
            return 0; // ACTION: CONTINUE
        }
    }
}

