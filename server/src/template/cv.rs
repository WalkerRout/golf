use askama::Template;

use crate::model::cv::*;

#[derive(Template)]
#[template(path = "cv.html")]
pub struct Cv {
  pub phone_number: String,
  pub email_address: String,
  pub website_link: String,
  pub github_link: String,
  pub linkedin_link: String,
  pub skills: TechnicalSkills,
  pub experience: ProfessionalExperience,
  pub education: Education,
  pub projects: NotableProjects,
}

impl Default for Cv {
  fn default() -> Self {
    Self {
      phone_number: "+1.604.652.8042".into(),
      email_address: "walkerrout04@gmail.com".into(),
      website_link: "https://walker.rout.ca/".into(),
      github_link: "https://github.com/WalkerRout/".into(),
      linkedin_link: "https://linkedin.com/in/walkerrout".into(),
      skills: Default::default(),
      experience: Default::default(),
      education: Default::default(),
      projects: Default::default(),
    }
  }
}

impl From<Builder> for Cv {
  fn from(builder: Builder) -> Self {
    Self {
      skills: builder.skills,
      experience: builder.experience,
      education: builder.education,
      projects: builder.projects,
      ..Default::default()
    }
  }
}
