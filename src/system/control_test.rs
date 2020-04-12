use crate::rabbitmq;
use crate::system::constants;
use crate::system::processes::Processes;
use crate::Control;
use log::Level;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    #[should_panic]
    fn test_get_time_none_letters() {
        let mut controller = Control::new();
        let time = controller.get_time();
        let result = time.chars().all(char::is_alphanumeric);
        assert!(result);
    }

    #[test]
    fn test_time_contains() {
        let mut controller = Control::new();
        let time = controller.get_time();
        if !time.contains("-") & !time.contains(":") {
            assert!(false);
        }
    }

    #[test]
    fn control_loop_exit() {
        let mut controller = Control::new();
        controller.set_shutdown();
        let time = controller.control_loop();
        assert_eq!(controller.get_event_counter(), 0);
    }
}
