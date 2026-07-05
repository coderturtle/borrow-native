use std::collections::HashMap;

/// A single customer order. See `SPEC.md` for the exercise this type is used in.
pub struct Order {
    pub customer: String,
    pub total_cents: u64,
}

/// Merge order totals by customer name, consuming `orders`.
///
/// Edge cases this must handle (see `SPEC.md` for the full list):
/// empty input, a single order, duplicate customers (same case) summing
/// into one entry, multiple distinct customers not leaking totals across
/// each other, case-sensitive customer names, and zero-cent orders still
/// producing an entry.
pub fn merge_customer_totals(orders: Vec<Order>) -> HashMap<String, u64> {
    let mut totals: HashMap<String, u64> = HashMap::new();
    for order in orders {
        *totals.entry(order.customer).or_insert(0) += order.total_cents;
    }
    totals
}
