#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heading {
    N, // 北，Y 轴正方向
    S, // 南，Y 轴负方向
    E, // 东，X 轴正方向
    W, // 西，X 轴负方向
}

impl Heading {
    /// 左转90度
    pub fn turn_left(&self) -> Self {
        match self {
            Heading::N => Heading::W,
            Heading::W => Heading::S,
            Heading::S => Heading::E,
            Heading::E => Heading::N,
        }
    }

    /// 右转90度
    pub fn turn_right(&self) -> Self {
        match self {
            Heading::N => Heading::E,
            Heading::E => Heading::S,
            Heading::S => Heading::W,
            Heading::W => Heading::N,
        }
    }
}

impl From<char> for Heading {
    fn from(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'N' => Heading::N,
            'S' => Heading::S,
            'E' => Heading::E,
            'W' => Heading::W,
            _ => panic!("无效的朝向字符，只能是 N, S, E, W"),
        }
    }
}

/// 执行器组件（支持倒车和加速）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Executor {
    x: i32,
    y: i32,
    heading: Heading,
    reverse_mode: bool, // 是否处于倒车状态
    boost_mode: bool,   // 是否处于加速状态
}

impl Executor {
    /// 默认初始化（位置0,0，朝向N，无特殊模式）
    pub fn new() -> Self {
        Self::default()
    }

    /// 自定义初始化
    pub fn init(x: i32, y: i32, heading: char) -> Self {
        Executor {
            x,
            y,
            heading: Heading::from(heading),
            reverse_mode: false,
            boost_mode: false,
        }
    }

    /// 执行单条指令
    /// 
    /// # 支持的指令
    /// - `M`: 移动（根据模式决定前进/后退及步数）
    /// - `L`: 转向（根据模式决定是否先移动及转向方向）
    /// - `R`: 转向（根据模式决定是否先移动及转向方向）
    /// - `B`: 切换倒车状态（开/关）
    /// - `F`: 切换加速状态（开/关）
    pub fn execute(&mut self, command: char) -> Result<(), &'static str> {
        match command {
            'M' => {
                let steps = if self.boost_mode { 2 } else { 1 };
                let steps = if self.reverse_mode { -steps } else { steps };
                self.move_steps(steps);
                Ok(())
            }
            'L' => {
                // 加速状态下先移动一格（方向由倒车模式决定）
                if self.boost_mode {
                    let steps = if self.reverse_mode { -1 } else { 1 };
                    self.move_steps(steps);
                }
                // 转向：倒车模式下左右互换
                if self.reverse_mode {
                    self.turn_right();
                } else {
                    self.turn_left();
                }
                Ok(())
            }
            'R' => {
                if self.boost_mode {
                    let steps = if self.reverse_mode { -1 } else { 1 };
                    self.move_steps(steps);
                }
                if self.reverse_mode {
                    self.turn_left();
                } else {
                    self.turn_right();
                }
                Ok(())
            }
            'B' => {
                self.reverse_mode = !self.reverse_mode;
                Ok(())
            }
            'F' => {
                self.boost_mode = !self.boost_mode;
                Ok(())
            }
            _ => Err("无效指令，只支持 M, L, R, B, F"),
        }
    }

    /// 批量执行指令
    pub fn execute_batch(&mut self, commands: &str) -> Result<(), &'static str> {
        for c in commands.chars() {
            self.execute(c)?;
        }
        Ok(())
    }

    // ---------- 内部辅助函数 ----------
    /// 沿当前朝向移动指定步数（正数前进，负数后退）
    fn move_steps(&mut self, steps: i32) {
        match self.heading {
            Heading::N => self.y += steps,
            Heading::S => self.y -= steps,
            Heading::E => self.x += steps,
            Heading::W => self.x -= steps,
        }
    }

    /// 左转90度（位置不变）
    fn turn_left(&mut self) {
        self.heading = self.heading.turn_left();
    }

    /// 右转90度（位置不变）
    fn turn_right(&mut self) {
        self.heading = self.heading.turn_right();
    }

    // ---------- 查询接口 ----------
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn heading(&self) -> Heading {
        self.heading
    }

    pub fn heading_char(&self) -> char {
        match self.heading {
            Heading::N => 'N',
            Heading::S => 'S',
            Heading::E => 'E',
            Heading::W => 'W',
        }
    }

    pub fn state(&self) -> (i32, i32, char) {
        (self.x, self.y, self.heading_char())
    }

    /// 查询当前模式状态（用于测试）
    pub fn modes(&self) -> (bool, bool) {
        (self.reverse_mode, self.boost_mode)
    }
}

impl Default for Executor {
    fn default() -> Self {
        Executor {
            x: 0,
            y: 0,
            heading: Heading::N,
            reverse_mode: false,
            boost_mode: false,
        }
    }
}

fn main() {
    let mut exec = Executor::new();
    println!("初始: {:?}", exec.state());
    exec.execute_batch("B F M L R M B F").unwrap();
    println!("执行后: {:?}", exec.state());
}
