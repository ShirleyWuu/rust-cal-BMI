// build calculator for BMI

pub fn calculate_bmi(weight: f32, height: f32) -> f32 {
    weight / (height * height)
}
