use ledger::{merge_customer_totals, Order};

#[test]
fn empty_input_produces_empty_map() {
    let result = merge_customer_totals(vec![]);
    assert!(result.is_empty());
}

#[test]
fn single_order_produces_one_entry() {
    let orders = vec![Order { customer: "Alice".to_string(), total_cents: 500 }];
    let result = merge_customer_totals(orders);
    assert_eq!(result.get("Alice"), Some(&500));
    assert_eq!(result.len(), 1);
}

#[test]
fn duplicate_customer_same_case_sums() {
    let orders = vec![
        Order { customer: "Bob".to_string(), total_cents: 300 },
        Order { customer: "Bob".to_string(), total_cents: 700 },
    ];
    let result = merge_customer_totals(orders);
    assert_eq!(result.get("Bob"), Some(&1000));
    assert_eq!(result.len(), 1);
}

#[test]
fn multiple_distinct_customers_do_not_leak_totals() {
    let orders = vec![
        Order { customer: "Alice".to_string(), total_cents: 100 },
        Order { customer: "Bob".to_string(), total_cents: 200 },
        Order { customer: "Alice".to_string(), total_cents: 50 },
    ];
    let result = merge_customer_totals(orders);
    assert_eq!(result.get("Alice"), Some(&150));
    assert_eq!(result.get("Bob"), Some(&200));
    assert_eq!(result.len(), 2);
}

#[test]
fn customer_names_are_case_sensitive() {
    let orders = vec![
        Order { customer: "Alice".to_string(), total_cents: 100 },
        Order { customer: "alice".to_string(), total_cents: 100 },
    ];
    let result = merge_customer_totals(orders);
    assert_eq!(result.get("Alice"), Some(&100));
    assert_eq!(result.get("alice"), Some(&100));
    assert_eq!(result.len(), 2);
}

#[test]
fn zero_cent_order_still_creates_entry() {
    let orders = vec![Order { customer: "Carol".to_string(), total_cents: 0 }];
    let result = merge_customer_totals(orders);
    assert_eq!(result.get("Carol"), Some(&0));
    assert_eq!(result.len(), 1);
}
