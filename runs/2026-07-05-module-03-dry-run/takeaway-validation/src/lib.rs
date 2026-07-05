//! Fixed per `enum-modeling-playbook`'s step 3 (see diff.patch for the naive starting point).

pub enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
    Cancelled,
}

pub enum RefundPolicy {
    FullRefund,
    PartialRefund,
    NoRefund,
}

pub fn refund_policy(status: &OrderStatus) -> RefundPolicy {
    match status {
        OrderStatus::Pending => RefundPolicy::FullRefund,
        OrderStatus::Cancelled => RefundPolicy::FullRefund,
        OrderStatus::Shipped => RefundPolicy::PartialRefund,
        OrderStatus::Delivered => RefundPolicy::NoRefund,
    }
}
