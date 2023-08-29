use std::fs;
use toml::Value;
fn init_from_toml() -> (f64, f64, f64) {
    let toml_content = fs::read_to_string("condition.toml").expect("Unable to read file");
    let parsed_content: Value = toml::from_str(&toml_content).expect("Unable to parse TOML");
    let toml_value = Value::try_from(parsed_content).unwrap();
    //print!("{:?}", toml_value);
    // 初始化參數
    let init_success_rate = toml_value["condition"]["init_success_rate"]
        .as_float()
        .unwrap(); // 初始成功率
    let success_increase = toml_value["condition"]["success_increase"]
        .as_float()
        .unwrap(); // 成功率增加量
    let single_shot_cost = toml_value["condition"]["single_shot_cost"]
        .as_float()
        .unwrap(); // 單次升級消耗

    (init_success_rate, success_increase, single_shot_cost)
}

fn calculate_expected_cost(
    accumulated_success_rate: f64,
    chance_to_this_stage: f64,
    failure_increase: f64,
    expected_trial_times: f64,
    current_loop: i32,
) -> f64 {
    println!(
        "第{}次升級，成功率: {:.4}%，目前累計{:.4}",
        current_loop,
        accumulated_success_rate * 100.0,
        expected_trial_times
    );
    // 計算每輪的期望值 = 進入此輪機率 * 本次成功機率 * 迴圈數
    // 進入此輪機率 =
    println!("進入本輪機率: {:.4}%", chance_to_this_stage * 100.0);
    println!("本輪成功機率: {:.4}%", accumulated_success_rate * 100.0);
    let expected_cost = chance_to_this_stage * accumulated_success_rate * current_loop as f64; // 此輪中止的次數期望值
    println!("成功的期望值: {:.4}", expected_cost);
    let current_cost = expected_trial_times + expected_cost; // 累計消耗 + 本次消耗
    if accumulated_success_rate >= 1.0 {
        println!("已達到100%成功率，不會再失敗");
        println!("累計消耗: {:.4}", current_cost);
        return current_cost;
    }
    let failure_rate = 1.00 - accumulated_success_rate as f64; // 此輪失敗率
    let rate_to_next_stage = chance_to_this_stage * failure_rate; // 進入下一輪機率
    println!("此輪進入下輪機率: {:.4}%", failure_rate * 100.0);
    println!("全域進入下輪機率: {:.4}%", rate_to_next_stage * 100.0);
    // Recursively calculate the expected cost for the next upgrade
    let next_round_rate = accumulated_success_rate + failure_increase; // 下一輪成功率
    calculate_expected_cost(
        next_round_rate,
        rate_to_next_stage,
        failure_increase,
        current_cost,
        current_loop + 1,
    )
}

fn main() {
    // 改從toml讀取
    let (init_success_rate, success_increase, single_shot_cost) = init_from_toml();
    let expected_trial_times = 0.0; // 預期試驗次數
    let current_loop: i32 = 1; // 當前次數

    let expected_cost = calculate_expected_cost(
        init_success_rate,
        1.0,
        success_increase,
        expected_trial_times,
        current_loop,
    );
    println!(
        "次數期望值: {:.4}，總共消耗的期望值: {:.4}",
        expected_cost,
        expected_cost * single_shot_cost as f64
    );
}
