struct Bank {
    balance: Vec<i64>,
}

impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self { balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let (account1, account2) = (account1 as usize - 1, account2 as usize - 1);

        if self.not_valid(account1)
            || self.not_valid(account2)
            || self.not_sufficient_balance(account1, money)
        {
            return false;
        }

        self.balance[account1] -= money;
        self.balance[account2] += money;

        true
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let account = account as usize - 1;

        if self.not_valid(account) {
            return false;
        }

        self.balance[account] += money;
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let account = account as usize - 1;

        if self.not_valid(account) || self.not_sufficient_balance(account, money) {
            return false;
        }

        self.balance[account] -= money;
        true
    }

    #[inline(always)]
    fn not_valid(&self, account: usize) -> bool {
        account >= self.balance.len()
    }

    #[inline(always)]
    fn not_sufficient_balance(&self, account: usize, money: i64) -> bool {
        self.balance[account] < money
    }
}
