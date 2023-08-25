fn calculate_expected_cost(
    accumulated_success_rate: f64,
    chance_to_this_stage: f64,
    failure_increase: f64,
    single_shot_cost: f64,
    current_cost: f64,
    current_loop: i32,
) -> f64 {
    println!(
        "第{}次升級，成功率: {:.2}%，目前累計{:.4}",
        current_loop,
        accumulated_success_rate * 100.0,
        current_cost
    );
    if accumulated_success_rate >= 1.0 {
        println!("已達到100%成功率，不會再失敗");
        println!("累計消耗: {:.2}", current_cost);
        return current_cost;
    }

    let failure_rate = 1.00 - accumulated_success_rate as f64;
    let current_stage_cost = single_shot_cost * current_loop as f64 * chance_to_this_stage;
    println!("成功機率: {:.2}%", accumulated_success_rate * 100.0);
    println!("本次消耗: {:.2}", current_stage_cost);
    let expected_cost = accumulated_success_rate * current_stage_cost; // 成功的期望值
    println!("成功的期望值: {:.2}", expected_cost);
    let current_cost = current_cost + expected_cost; // 累計消耗 + 本次消耗

    // Recursively calculate the expected cost for the next upgrade
    let next_round_rate = accumulated_success_rate + failure_increase; // 進入下一輪機率
    calculate_expected_cost(
        next_round_rate,
        failure_rate * chance_to_this_stage,
        failure_increase,
        single_shot_cost,
        current_cost,
        current_loop + 1,
    )
}

fn main() {
    let init_success_rate = 0.5; // 初始成功率
    let success_increase = 0.16; // 成功率增加量
    let single_shot_cost: f64 = 10.0; // 單次升級消耗
    let current_cost: f64 = 0.0; // 當前消耗
    let current_loop: i32 = 1; // 當前升級次數

    let expected_cost = calculate_expected_cost(
        init_success_rate,
        1.0,
        success_increase,
        single_shot_cost,
        current_cost,
        current_loop,
    );
    println!("總共消耗的期望值: {:.4}", expected_cost);
}
