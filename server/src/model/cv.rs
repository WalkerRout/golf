#![allow(dead_code)]

use crate::model::Build;

#[derive(Debug, Default)]
pub struct Builder {
  pub skills: TechnicalSkills,
  pub experience: ProfessionalExperience,
  pub education: Education,
  pub projects: NotableProjects,
}

impl Builder {
  pub fn add_language(mut self, lang: impl Into<String>) -> Self {
    self.skills.programming_languages.push(lang.into());
    self
  }

  pub fn add_technology(mut self, tech: impl Into<String>) -> Self {
    self.skills.technologies_frameworks.push(tech.into());
    self
  }

  pub fn add_database_or_tool(mut self, db: impl Into<String>) -> Self {
    self.skills.databases_tools.push(db.into());
    self
  }

  pub fn add_specialised(mut self, spec: impl Into<String>) -> Self {
    self.skills.specialised_skills.push(spec.into());
    self
  }

  pub fn add_job(mut self, job: impl Into<Job>) -> Self {
    self.experience.entries.push(job.into());
    self
  }

  pub fn add_education(mut self, qual: impl Into<Qualification>) -> Self {
    self.education.entries.push(qual.into());
    self
  }

  pub fn add_project(mut self, proj: impl Into<Project>) -> Self {
    self.projects.entries.push(proj.into());
    self
  }
}

// give ourselves the `build` function
impl Build for Builder {}

#[derive(Debug, Default, Clone)]
pub struct TechnicalSkills {
  pub programming_languages: Vec<String>,
  pub technologies_frameworks: Vec<String>,
  pub databases_tools: Vec<String>,
  pub specialised_skills: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct ProfessionalExperience {
  pub entries: Vec<Job>,
}

#[derive(Debug, Default, Clone)]
pub struct Job {
  pub title: String,
  pub company: String,
  pub location: Option<String>,
  // both of these should be timestamps
  pub start: Option<String>,
  pub end: Option<String>,
  pub notes: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct Education {
  pub entries: Vec<Qualification>,
}

#[derive(Debug, Default, Clone)]
pub struct Qualification {
  pub title: String,
  pub provider: String,
  pub description: Option<String>,
  pub notes: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct NotableProjects {
  pub entries: Vec<Project>,
}

#[derive(Debug, Default, Clone)]
pub struct Project {
  pub title: String,
  pub organisation: Option<String>,
  pub description: Option<String>,
  pub tools_used: Vec<String>,
}
