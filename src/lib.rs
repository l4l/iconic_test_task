#![feature(test)]
pub type Price = i32;
pub type Size = u32;
pub type Meta = u128;
pub type Order = (Size, Meta);
pub type Orders = Vec<Order>;

/// List with first ordering preserved by Pricing
/// then FIFO is preserved by Size
#[derive(Debug, Clone, Default)]
pub struct List {
    pub orders: Vec<(Price, Orders)>,
}

impl List {
    pub fn empty() -> List {
        List::from(vec![])
    }

    pub fn from(v: Vec<(Price, Orders)>) -> List {
        List { orders: v }
    }

    pub fn split(&mut self, issue: (Price, Size)) -> List {
        let (price, mut left) = issue;
        let mut orders = Vec::new();
        while let Some(mut order) = self.orders.pop() {
            if order.0 > price || left == 0 {
                break;
            }
            let mut row = List::orders_split(&mut order.1, &mut left);
            orders.push((order.0, row));
            if order.1.len() != 0 {
                self.orders.push(order);
                break;
            }
        }
        List::from(orders.into_iter().rev().collect())
    }

    pub fn add(&mut self, el: (Price, Size, Meta)) {
        match self.find(&el.0) {
            Ok(o) => self.orders[o].1.push((el.1, el.2)),
            Err(e) => self.orders.insert(e, (el.0, vec![(el.1, el.2)])),
        };
    }

    fn find(&self, el: &Price) -> Result<usize, usize> {
        self.orders.as_slice().binary_search_by(
            |val| val.0.cmp(&el).reverse(),
        )
    }

    fn order_split(order: Order, left: &mut Size) -> Result<(Order, Order), Order> {
        use std::cmp;
        let min = cmp::min(*left, order.0);
        *left -= min;
        if order.0 - min == 0 {
            Err((min, order.1))
        } else {
            Ok(((min, order.1), (order.0 - min, order.1)))
        }
    }

    fn orders_split(els: &mut Orders, left: &mut Size) -> Vec<Order> {
        let mut v = Vec::new();
        while *left != 0 {
            let el = els.pop();
            match el.map(|el| List::order_split(el, left)) {
                Some(Ok((a, b))) => {
                    v.push(a);
                    els.push(b);
                    break;
                }
                Some(Err(a)) => {
                    v.push(a);
                }
                None => break,
            }
        }
        v
    }
}

fn main() {}

mod test {
    use super::*;
    #[test]
    fn test_search() {
        let mut l = List { orders: vec![(7, vec![(10, 20)]), (5, vec![(10, 20)])] };
        l.add((5, 5, 1));
        assert_eq!(
            l.orders,
            vec![(7, vec![(10, 20)]), (5, vec![(10, 20), (5, 1)])]
        );
        l.add((6, 10, 2));
        assert_eq!(
            l.orders,
            vec![
                (7, vec![(10, 20)]),
                (6, vec![(10, 2)]),
                (5, vec![(10, 20), (5, 1)]),
            ]
        );
        l.add((7, 20, 3));
        assert_eq!(
            l.orders,
            vec![
                (7, vec![(10, 20), (20, 3)]),
                (6, vec![(10, 2)]),
                (5, vec![(10, 20), (5, 1)]),
            ]
        );
        l.add((8, 30, 4));
        assert_eq!(
            l.orders,
            vec![
                (8, vec![(30, 4)]),
                (7, vec![(10, 20), (20, 3)]),
                (6, vec![(10, 2)]),
                (5, vec![(10, 20), (5, 1)]),
            ]
        );
    }

    #[test]
    fn test_order_split_mid() {
        let v: Order = (5, 3);
        let mut left = 4;
        assert_eq!(List::order_split(v, &mut left), Ok(((4, 3), (1, 3))));
        assert_eq!(left, 0);
    }

    #[test]
    fn test_order_split_full() {
        let v: Order = (5, 3);
        let mut left = 5;
        assert_eq!(List::order_split(v, &mut left), Err((5, 3)));
        assert_eq!(left, 0);
    }

    #[test]
    fn test_order_split_part() {
        let v: Order = (5, 3);
        let mut left = 7;
        assert_eq!(List::order_split(v, &mut left), Err((5, 3)));
        assert_eq!(left, 2);
    }

    #[test]
    fn test_orders_split() {
        let mut els = vec![(20, 1), (15, 2), (3, 4), (10, 3)];
        let mut left = 15;
        let v = List::orders_split(&mut els, &mut left);

        assert_eq!(v, vec![(10, 3), (3, 4), (2, 2)]);
        assert_eq!(els, vec![(20, 1), (13, 2)]);
        assert_eq!(left, 0);
    }

    #[test]
    fn test_split() {
        let mut lst = List {
            orders: vec![
                (7, vec![(900, 30), (1000, 35)]),
                (6, vec![(700, 20), (800, 25)]),
                (5, vec![(500, 10), (600, 15)]),
            ],
        };

        assert_eq!(
            lst.split((6, 1400)).orders,
            vec![(6, vec![(300, 25)]), (5, vec![(600, 15), (500, 10)])]
        );
        assert_eq!(
            lst.orders,
            vec![
                (7, vec![(900, 30), (1000, 35)]),
                (6, vec![(700, 20), (500, 25)]),
            ]
        );
    }
}
