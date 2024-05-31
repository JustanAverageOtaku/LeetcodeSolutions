use std::cell::RefCell;

struct StockSpanner {
    prices: RefCell<Vec<i32>>
}

impl StockSpanner {

    fn new() -> Self {
        StockSpanner { prices: RefCell::new(Vec::new()) }
    }
    
    fn next(&self, price: i32) -> i32 {
        let mut count = 1;
        for i in self.prices.borrow().iter().rev() {
            if *i > price {
                break;
            }

            count += 1;
        }
        
        self.prices.borrow_mut().push(price);
        
        count
    }
}
