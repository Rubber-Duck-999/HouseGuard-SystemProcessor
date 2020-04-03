use crate::system::constants;
use crate::system::processes::Processes;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_check_exists() {
        let mut process_check = Processes::new();
        let mut test_runner_sh = "./deploy/runTest.sh";
        let mut found = process_check.ps_find(test_runner_sh);
        process_check.kill_main_component(test_runner_sh);
        found = process_check.ps_find(test_runner_sh);
        assert_eq!(found, 0);
    }

    #[test]
    fn test_start_runner() {
        let mut process_check = Processes::new();
        let mut test_runner_sh = "./deploy/runTest.sh";
        process_check.start_process(test_runner_sh);
        let mut found = process_check.ps_find(test_runner_sh);
        process_check.kill_main_component(test_runner_sh);
        found = process_check.ps_find(test_runner_sh);
        assert_eq!(found, 0);
    }

    #[test]
    fn test_start_process() {
        let mut process_check = Processes::new();
        let test_runner_sh = constants::DEPLOY_SCRIPTS.to_owned() + &constants::FH_EXE.to_owned();
        process_check.start_process(&test_runner_sh);
        let mut found = process_check.ps_find(&test_runner_sh);
        process_check.kill_main_component(&test_runner_sh);
        found = process_check.ps_find(&test_runner_sh);
        assert_eq!(found, 0);
    }

    #[test]
    fn test_start_two_runner() {
        let mut process_check = Processes::new();
        let mut test_runner_sh = "./deploy/runTest.sh";
        process_check.start_process(test_runner_sh);
        process_check.start_process(test_runner_sh);
        process_check.start_process(test_runner_sh);
        let mut found = process_check.ps_find(test_runner_sh);
        process_check.kill_main_component(test_runner_sh);
        found = process_check.ps_find(test_runner_sh);
        assert_eq!(found, 0);
    }

    #[test]
    fn kill_previous_three() {
        let mut process_check = Processes::new();
        let mut test_runner_sh = "./deploy/runTest.sh";
        let mut found = process_check.ps_find(test_runner_sh);
        process_check.kill_main_component(test_runner_sh);
        found = process_check.ps_find(test_runner_sh);
        assert_eq!(found, 0);
    }

    #[test]
    fn test_start_runner_duplicate() {
        let mut process_check = Processes::new();
        let mut test_runner_sh = "./deploy/runTest.sh";
        process_check.start_process(test_runner_sh);
        process_check.start_process(test_runner_sh);
        let mut found = process_check.ps_find(test_runner_sh);
        process_check.kill_duplicate_component(test_runner_sh);
        found = process_check.ps_find(test_runner_sh);
        assert_eq!(found, 1);
    }

    #[test]
    fn test_kill_duplicate() {
        let mut process_check = Processes::new();
        let mut test_runner_sh = "./deploy/runTest.sh";
        process_check.start_process(test_runner_sh);
        process_check.start_process(test_runner_sh);
        let mut found = process_check.ps_find(test_runner_sh);
        process_check.kill_component(test_runner_sh, false);
        found = process_check.ps_find(test_runner_sh);
        assert_eq!(found, 1);
    }

    #[test]
    fn test_kill_main() {
        let mut process_check = Processes::new();
        let mut test_runner_sh = "./deploy/runTest.sh";
        process_check.start_process(test_runner_sh);
        let mut found = process_check.ps_find(test_runner_sh);
        assert_eq!(found, 1);
        process_check.kill_component(test_runner_sh, false);
        found = process_check.ps_find(test_runner_sh);
        assert_eq!(found, 0);
    }

    #[test]
    fn test_restart() {
        let mut process_check = Processes::new();
        let mut test_runner_sh = "./deploy/runTest.sh";
        let mut found = process_check.ps_find(test_runner_sh);
        assert_eq!(found, 0);
        process_check.kill_component(test_runner_sh, true);
        found = process_check.ps_find(test_runner_sh);
        assert_eq!(found, 1);
        process_check.kill_component(test_runner_sh, false);
        found = process_check.ps_find(test_runner_sh);
        assert_eq!(found, 0);
    }

    #[test]
    #[should_panic]
    fn test_error_pid_exist() {
        let mut process_check = Processes::new();
        let pid: u32 = 5409;
        assert!(process_check.ps_find_pid(pid));
    }
}
