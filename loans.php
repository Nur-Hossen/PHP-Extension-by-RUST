<?php
try {
    $loan_amount = 100000.00;
    $interest_rate = 12.00; // 12%
    $loan_type = "reducing";
    $installments = 12;
    $disburse_date = "2026-02-01";

    // Rust এক্সটেনশনের ফাংশন কল
    $schedule = calculate_loan_schedule(
        $loan_amount, 
        $interest_rate, 
        $loan_type, 
        $installments, 
        $disburse_date
    );

    echo "<pre>";  
    echo "Loan schedule: ".   PHP_EOL;
    print_r($schedule); echo PHP_EOL;
} catch (Exception $e) {
    echo "Error: " . $e->getMessage();
}
?>

