use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Position {
    pub symbol: String,
    pub quantity: f64,
    pub entry_price: f64,
    pub current_price: f64,
}

impl Position {
    pub fn pnl(&self) -> f64 {
        (self.current_price - self.entry_price) * self.quantity
    }

    pub fn pnl_percent(&self) -> f64 {
        ((self.current_price - self.entry_price) / self.entry_price) * 100.0
    }
}

#[derive(Debug)]
pub struct RiskGuard {
    positions: HashMap<String, Position>,
    max_position_size: f64,
    max_loss_percent: f64,
}

impl RiskGuard {
    pub fn new(max_position_size: f64, max_loss_percent: f64) -> Self {
        Self {
            positions: HashMap::new(),
            max_position_size,
            max_loss_percent,
        }
    }

    pub fn add_position(&mut self, position: Position) -> Result<(), String> {
        if position.quantity > self.max_position_size {
            return Err(format!(
                "Position size {} exceeds maximum {}",
                position.quantity, self.max_position_size
            ));
        }
        self.positions.insert(position.symbol.clone(), position);
        Ok(())
    }

    pub fn update_price(&mut self, symbol: &str, price: f64) {
        if let Some(pos) = self.positions.get_mut(symbol) {
            pos.current_price = price;
        }
    }

    pub fn check_stop_loss(&self) -> Vec<String> {
        self.positions
            .values()
            .filter(|p| p.pnl_percent() < -self.max_loss_percent)
            .map(|p| p.symbol.clone())
            .collect()
    }

    pub fn total_pnl(&self) -> f64 {
        self.positions.values().map(|p| p.pnl()).sum()
    }

    pub fn calculate_var(&self, confidence: f64) -> f64 {
        // Simplified VaR calculation
        let total_value: f64 = self.positions
            .values()
            .map(|p| p.quantity * p.current_price)
            .sum();
        total_value * (1.0 - confidence) * 0.1
    }
}

fn main() {
    println!("🛡️  RiskGuard - Advanced Risk Management System");
    println!("===============================================\n");

    let mut guard = RiskGuard::new(100.0, 5.0);

    // Add positions
    let pos1 = Position {
        symbol: "BTCUSD".to_string(),
        quantity: 10.0,
        entry_price: 50000.0,
        current_price: 51000.0,
    };

    let pos2 = Position {
        symbol: "ETHUSD".to_string(),
        quantity: 50.0,
        entry_price: 3000.0,
        current_price: 2900.0,
    };

    guard.add_position(pos1).unwrap();
    guard.add_position(pos2).unwrap();

    println!("📊 Portfolio Analysis:");
    for (symbol, pos) in &guard.positions {
        println!("\n  {}:", symbol);
        println!("    Quantity: {}", pos.quantity);
        println!("    Entry: ${:.2}", pos.entry_price);
        println!("    Current: ${:.2}", pos.current_price);
        println!("    P&L: ${:.2} ({:.2}%)", pos.pnl(), pos.pnl_percent());
    }

    println!("\n💰 Total P&L: ${:.2}", guard.total_pnl());
    println!("📉 VaR (95%): ${:.2}", guard.calculate_var(0.95));

    let stop_losses = guard.check_stop_loss();
    if !stop_losses.is_empty() {
        println!("\n⚠️  Stop Loss Triggered: {:?}", stop_losses);
    } else {
        println!("\n✅ All positions within risk limits");
    }
}
