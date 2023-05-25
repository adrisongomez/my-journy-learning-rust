pub mod people;
use people::people as p;

fn print_team_size<T>(supervisor: &T)
where
    T: p::Supervisor,
{
    println!("Team size: {}", supervisor.get_team_size());
}

fn print_language(coder: &impl p::Coder)
{
    println!("Language: {}", coder.get_language());
}

fn print_employee_badge(employee: &impl p::Employee)
{
    println!("Employee badge: {}", employee.get_badge_number());
}

fn main() {
    let manager = p::Manager {
        job_id: "123".to_string(),
        badge_number: "456".to_string(),
        titular_name: "John Doe".to_string(),
    };
    let developer = p::Developer {
        job_id: "789".to_string(),
        badge_number: "012".to_string(),
        titular_name: "Jane Doe".to_string(),
    };
    print_team_size(&manager);
    print_language(&developer);
    print_employee_badge(&manager);
    print_employee_badge(&developer)
}
