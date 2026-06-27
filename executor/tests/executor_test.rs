#[cfg(test)]
mod tests {
    use super::*;

    // ---------- 原有测试（确保向后兼容） ----------
    #[test]
    fn test_default_initialization() {
        let exec = Executor::new();
        assert_eq!(exec.position(), (0, 0));
        assert_eq!(exec.heading_char(), 'N');
        assert_eq!(exec.state(), (0, 0, 'N'));
        assert_eq!(exec.modes(), (false, false));
    }

    #[test]
    fn test_custom_initialization() {
        let exec = Executor::init(5, 3, 'E');
        assert_eq!(exec.position(), (5, 3));
        assert_eq!(exec.heading_char(), 'E');
        assert_eq!(exec.modes(), (false, false));
    }

    #[test]
    fn test_move_forward() {
        let mut exec = Executor::init(0, 0, 'N');
        exec.execute('M').unwrap();
        assert_eq!(exec.state(), (0, 1, 'N'));
    }

    #[test]
    fn test_turn_left_right() {
        let mut exec = Executor::new();
        exec.execute('L').unwrap();
        assert_eq!(exec.state(), (0, 0, 'W'));
        exec.execute('R').unwrap();
        assert_eq!(exec.state(), (0, 0, 'N'));
    }

    #[test]
    fn test_batch_commands_old() {
        let mut exec = Executor::init(0, 0, 'N');
        exec.execute_batch("MMRMMLMM").unwrap();
        assert_eq!(exec.state(), (2, 4, 'N'));
    }

    #[test]
    fn test_invalid_command() {
        let mut exec = Executor::new();
        let result = exec.execute('X');
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "无效指令，只支持 M, L, R, B, F");
    }

    // ---------- 新增测试：倒车模式 ----------
    #[test]
    fn test_reverse_mode_toggle() {
        let mut exec = Executor::new();
        // 开启倒车
        exec.execute('B').unwrap();
        assert_eq!(exec.modes(), (true, false));
        // 再执行一次关闭
        exec.execute('B').unwrap();
        assert_eq!(exec.modes(), (false, false));
    }

    #[test]
    fn test_reverse_mode_move_backward() {
        let mut exec = Executor::init(0, 0, 'N');
        exec.execute('B').unwrap(); // 开启倒车
        exec.execute('M').unwrap(); // 应该后退一格
        assert_eq!(exec.state(), (0, -1, 'N'));
        // 转向不变
        exec.execute('M').unwrap();
        assert_eq!(exec.state(), (0, -2, 'N'));
    }

    #[test]
    fn test_reverse_mode_turn_swapped() {
        let mut exec = Executor::init(0, 0, 'N');
        exec.execute('B').unwrap();
        // L 应该变成右转
        exec.execute('L').unwrap();
        assert_eq!(exec.state(), (0, 0, 'E'));
        // R 应该变成左转
        exec.execute('R').unwrap();
        assert_eq!(exec.state(), (0, 0, 'N'));
    }

    #[test]
    fn test_reverse_mode_combined() {
        let mut exec = Executor::init(2, 3, 'E');
        exec.execute('B').unwrap();
        exec.execute_batch("MLMR").unwrap();
        // 期望：M后退 -> (1,3,E); L（实际右转）-> (1,3,S); M后退 -> (1,2,S); R（实际左转）-> (1,2,E)
        assert_eq!(exec.state(), (1, 2, 'E'));
    }

    // ---------- 新增测试：加速模式 ----------
    #[test]
    fn test_boost_mode_toggle() {
        let mut exec = Executor::new();
        exec.execute('F').unwrap();
        assert_eq!(exec.modes(), (false, true));
        exec.execute('F').unwrap();
        assert_eq!(exec.modes(), (false, false));
    }

    #[test]
    fn test_boost_mode_move_double() {
        let mut exec = Executor::init(0, 0, 'N');
        exec.execute('F').unwrap();
        exec.execute('M').unwrap();
        assert_eq!(exec.state(), (0, 2, 'N'));
    }

    #[test]
    fn test_boost_mode_turn_with_move() {
        let mut exec = Executor::init(0, 0, 'N');
        exec.execute('F').unwrap();
        // L: 前进1格，再左转
        exec.execute('L').unwrap();
        assert_eq!(exec.state(), (0, 1, 'W'));
        // R: 前进1格，再右转（当前朝西，前进则x-1，右转后朝北）
        exec.execute('R').unwrap();
        assert_eq!(exec.state(), (-1, 1, 'N'));
    }

    // ---------- 新增测试：叠加模式 ----------
    #[test]
    fn test_combined_mode_move_backward_double() {
        let mut exec = Executor::init(0, 0, 'N');
        exec.execute('B').unwrap();
        exec.execute('F').unwrap();
        // 叠加：M倒退2格
        exec.execute('M').unwrap();
        assert_eq!(exec.state(), (0, -2, 'N'));
    }

    #[test]
    fn test_combined_mode_turn_with_backward_move() {
        let mut exec = Executor::init(0, 0, 'N');
        exec.execute('B').unwrap();
        exec.execute('F').unwrap();
        // L: 倒退1格，然后右转（倒车时左右互换）
        exec.execute('L').unwrap();
        assert_eq!(exec.state(), (0, -1, 'E'));
        // R: 倒退1格，然后左转
        exec.execute('R').unwrap();
        assert_eq!(exec.state(), (0, -2, 'N'));
    }

    #[test]
    fn test_combined_mode_turn_with_boost_and_reverse() {
        let mut exec = Executor::init(5, 5, 'S');
        exec.execute('B').unwrap();
        exec.execute('F').unwrap();
        // 朝南，倒退1格（y+1），然后右转（倒车时左右互换，R变左转）-> 朝东
        exec.execute('R').unwrap();
        assert_eq!(exec.state(), (5, 6, 'E'));
        // 再L: 倒退1格（x-1），然后右转（倒车时L变右转）-> 朝南
        exec.execute('L').unwrap();
        assert_eq!(exec.state(), (4, 6, 'S'));
    }

    // ---------- 边界与混合测试 ----------
    #[test]
    fn test_toggle_multiple_times() {
        let mut exec = Executor::new();
        exec.execute_batch("BFBF").unwrap();
        // B开，F开，B关，F关 => 回到初始
        assert_eq!(exec.modes(), (false, false));
    }

    #[test]
    fn test_complex_sequence_with_modes() {
        let mut exec = Executor::init(0, 0, 'E');
        // 序列：B F M L R M B F
        exec.execute_batch("BFMLRMBF").unwrap();
        // 逐步验证：
        // B: reverse=true, boost=false
        // F: reverse=true, boost=true
        // M: 倒退2格 -> (-2,0,E)
        // L: 倒退1格，然后右转(倒车) -> (-3,0,S)
        // R: 倒退1格，然后左转(倒车) -> (-3,-1,E)
        // M: 倒退2格 -> (-5,-1,E)
        // B: reverse=false, boost=true
        // F: reverse=false, boost=false
        assert_eq!(exec.state(), (-5, -1, 'E'));
        assert_eq!(exec.modes(), (false, false));
    }

    #[test]
    fn test_move_steps_with_negative_coords() {
        let mut exec = Executor::init(-10, -10, 'W');
        exec.execute('F').unwrap();
        exec.execute('M').unwrap(); // 加速前进2格，朝西 => x减小2
        assert_eq!(exec.state(), (-12, -10, 'W'));
        exec.execute('B').unwrap();
        exec.execute('M').unwrap(); // 叠加倒退2格 => x增加2
        assert_eq!(exec.state(), (-10, -10, 'W'));
    }

    #[test]
    fn test_boost_mode_no_jump_but_effect_same() {
        // 加速模式下M前进2格，中间没有其他变化，效果等同于一次性移动2格。
        // 测试确保移动2格正确
        let mut exec = Executor::init(0, 0, 'N');
        exec.execute('F').unwrap();
        exec.execute('M').unwrap();
        assert_eq!(exec.position(), (0, 2));
        // 再次移动
        exec.execute('M').unwrap();
        assert_eq!(exec.position(), (0, 4));
    }
}
