use leptos::*;

#[derive(Clone)]
struct WorkExperience {
  company: &'static str,
  job_title: &'static str,
  icon: &'static str,
  start_date: &'static str,
  end_date: &'static str,
}

#[component]
fn WorkExperienceItem(exp: WorkExperience) -> impl IntoView {
  view! {
    <div class="work-item">
      <img src={exp.icon} alt={exp.company} class="work-icon"/>
      <div class="work-details">
        <div class="company">{exp.company}</div>
        <div class="job-title">{exp.job_title}</div>
      </div>
      <div class="dates">{format!("{} - {}", exp.start_date, exp.end_date)}</div>
    </div>
  }
}

#[component]
pub fn WorkSection() -> impl IntoView {
    // Define your work experiences
    let experiences = vec![
        WorkExperience {
            company: "Carbon",
            job_title: "Software Engineer",
            icon: "/public/carbon.png", // Example icon path
            start_date: "Apr 2022",
            end_date: "Present",
        },
    ];

    view! {
        <div class="work-section">
            <div class="section-header">Work</div>
            {experiences.into_iter().map(|exp| (WorkExperienceItemProps { exp: exp })).collect::<Vec<_>>()}
        </div>
    }
}