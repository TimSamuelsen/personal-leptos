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
          job_title: "Senior Software Engineer",
          icon: "/public/carbon.svg",
          start_date: "Apr 2023",
          end_date: "Present",
        },
        WorkExperience {
            company: "Carbon",
            job_title: "Software Engineer",
            icon: "/public/carbon.svg",
            start_date: "Apr 2022",
            end_date: "Apr 2023",
        },
        WorkExperience {
          company: "Stanford University",
          job_title: "Graduate Research Assistant",
          icon: "/public/stanford.svg",
          start_date: "Dec 2020",
          end_date: "Mar 2022",
        }
    ];

    view! {
        <div class="work-section">
            <div
              style="
                display: flex; /* Flex layout to align inline */
                align-items: flex-start;
                padding: 10px; /* Optional padding */
                border-bottom: 1px solid #444;
              "
            >
              <img 
                  src="/public/work_bag.svg" 
                  style="width: 30px;
                        height: 30px;
                        margin-right: 15px; /* Space between icon and text */"
              />
              <div class="section-header">Work</div>
            </div>
            {experiences.into_iter().map(|exp| (WorkExperienceItemProps { exp: exp })).collect::<Vec<_>>()}

            <div class="flex justify-between items-center w-full"
                
                 style="border-top: 1px solid #444; padding: 15px;"
            >
            <button
                onclick="window.open('/public/tim_resume.pdf', '_blank')"
                class="button-common"
                style="justify-content: center;"
            >
              Resume
              <img src="/public/arrow-up-right.svg" style="width: 22px; height: 22px; margin-left: 5px;"/>
            </button>
            </div>

        </div>
    }
}