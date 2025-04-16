use crate::model::cv::*;

pub fn builder() -> Builder {
  [
    build_languages,
    build_technologies,
    build_databases_tools,
    build_specialised,
    build_experience,
    build_education,
    build_projects,
  ]
  .into_iter()
  .fold(Builder::default(), |builder, step| step(builder))
}

fn build_languages(builder: Builder) -> Builder {
  builder
    .add_language("C")
    .add_language("C++20")
    .add_language("Rust")
    .add_language("Haskell")
    .add_language("Python")
    .add_language("JavaScript")
    .add_language("HTML/CSS")
    .add_language("SQL")
    .add_language("PHP")
    .add_language("C#")
    .add_language("Java")
    .add_language("Nim")
    .add_language("R")
    .add_language("Swift")
}

fn build_technologies(builder: Builder) -> Builder {
  builder
    .add_technology("Linux APIs")
    .add_technology("Tokio")
    .add_technology("Flask")
    .add_technology("FastAPI")
    .add_technology("Jupyter")
    .add_technology("LangChain")
    .add_technology("Unity")
    .add_technology("UnrealEngine")
    .add_technology("Xcode w/ AVP")
    .add_technology("JQuery")
    .add_technology("HTMX")
    .add_technology("Tailwind Css")
    .add_technology("Plotly.js")
    .add_technology("Azure")
}

fn build_databases_tools(builder: Builder) -> Builder {
  builder
    .add_database("Postgres")
    .add_database("MySQL")
    .add_database("SQLite")
    .add_database("Redis")
    .add_database("Elasticsearch")
    .add_database("Qdrant")
    .add_database("Neo4j")
    .add_database("Docker")
    .add_database("GitHub Actions")
    .add_database("Git")
    .add_database("Nginx")
}

fn build_specialised(builder: Builder) -> Builder {
  builder
    .add_specialised("Systems Programming")
    .add_specialised("Socket/Network Development")
    .add_specialised("Graphics Programming")
    .add_specialised("WebAssembly")
    .add_specialised("Machine Learning")
    .add_specialised("Embedded Systems (AVR)")
    .add_specialised("Linux Kernel Module Development")
    .add_specialised("Compiler Development")
    .add_specialised("2D/3D Game Development")
    .add_specialised("Software Development")
    .add_specialised("DevOps")
    .add_specialised("Teaching")
    .add_specialised("Problem Solving")
    .add_specialised("Communication")
}

fn build_experience(builder: Builder) -> Builder {
  builder
    .add_job(Job {
      title: "Student Software Developer".into(),
      company: "UBC Emerging Media Lab".into(),
      location: Some("Vancouver, BC".into()),
      start: Some("January 2024".into()),
      end: None,
      notes: vec![
        "Developed interactive applications leveraging emerging technologies to enhance user engagement.".into(),
        "Collaborated with interdisciplinary teams to design and implement innovative software solutions.".into(),
        "Contributed to projects that integrate virtual and augmented reality for educational purposes.".into(),
      ],
    })
    .add_job(Job {
      title: "Software Developer / Consultant".into(),
      company: "Planetworks Consulting".into(),
      location: Some("North Vancouver, BC".into()),
      start: Some("April 2018".into()),
      end: Some("September 2023".into()),
      notes: vec![
        "Designed and built front-end graphs for a UNICEF bandwidth monitor using Plotly.js, JQuery, Flask, and Jupyter".into(),
        "Constructed a phone line monitoring system for School District 36, implementing auto-responding school phone outage detection".into(),
        "Utilized tools including StarTrinity, SIPp, PostgreSQL, Python, and bash scripts for system development".into(),
        "Developed a bandwidth monitoring tool using Docker, Python/Flask, and JavaScript".into(),
        "Utilized technologies including JQuery and Plotly for data visualization".into(),
      ],
    })
    .add_job(Job {
      title: "Instructor".into(),
      company: "UTG Academy".into(),
      location: Some("Vancouver, BC".into()),
      start: Some("September 2021".into()),
      end: Some("July 2022".into()),
      notes: vec![
        "Instructed Unity game development and computer science concepts for students aged 8-13".into(),
        "Taught 2D and 3D game development using Unity for older students (10-13)".into(),
        "Guided younger students (8-11) in 2D game development using PixelPAD and Python".into(),
      ],
    })
    .add_job(Job {
      title: "Service Clerk".into(),
      company: "Sobeys".into(),
      location: Some("North Vancouver, BC".into()),
      start: Some("October 2019".into()),
      end: Some("April 2020".into()),
      notes: vec![
        "Coordinated incoming deliveries and performed quality checks".into(),
        "Managed inventory by stocking shelves, checking expiry dates, and maintaining display standards".into(),
        "Provided customer support and ensured cleanliness across store aisles".into(),
      ],
    })
}

fn build_education(builder: Builder) -> Builder {
  builder
    .add_education(Qualification {
      title: "BSc in Cognitive Systems".into(),
      provider: "University of British Columbia, BC".into(),
      description: Some("3rd Year in Progress".into()),
      notes: vec![],
    })
    .add_education(Qualification {
      title: "High School Diploma".into(),
      provider: "Handsworth Secondary School, North Vancouver, BC".into(),
      description: Some("Honours".into()),
      notes: vec![],
    })
    .add_education(Qualification {
      title: "Professional Development".into(),
      provider: "Various".into(),
      description: None,
      notes: vec![
        "Multiple Unity 2D and 3D C# Courses (Parts 1-4)".into(),
        "AP Computer Science (OOP and Java) Courses".into(),
        "Advanced/Competitive Computer Science Courses in Java, Python, and C++".into(),
      ],
    })
    .add_education(Qualification {
      title: "Autodidacticism".into(),
      provider: "Self-Directed Technical Learning".into(),
      description: None,
      notes: vec![
        "Dedicated to lifelong, curiosity-driven learning across computer science, mathematics, and electrical engineering".into(),
        "Maintain a curated index of 150+ technical blogs for continual exploration and motivation".into(),
        "Passionate about correctness by construction, leveraging algebraic methods and type theory to eliminate entire classes of bugs".into(),
        "Engaged in ongoing study of category theory, with recent focus on comonads and their role in compositional design (e.g., builders, zippers)".into(),
      ],
    })
}

fn build_projects(builder: Builder) -> Builder {
  builder
    .add_project(Project {
      title: "AR-A-SOP".into(),
      organisation: None,
      description: Some("Augmented-reality assisted standard operating procedures for the Martin T75 Sliding Table Saw".into()),
      tools_used: vec!["RealityKit".into(), "ARKit".into(), "Xcode".into(), "SwiftUI".into()],
    })
    .add_project(Project {
      title: "KG-RAG".into(),
      organisation: None,
      description: Some("Fullstack RAG tool utilizing knowledge graphs and vector stores to index UBC policies, enabling accurate and efficient policy compliance checks for new faculty project ideas".into()),
      tools_used: vec!["Knowledge Graphs".into(), "Vector Stores".into(), "Neo4j".into()],
    })
    .add_project(Project {
      title: "OCELIA".into(),
      organisation: None,
      description: Some("Fullstack ASP.NET Core Blazor RAG (Retrieval-Augmented Generation) application with an internal EML server using WebSockets".into()),
      tools_used: vec!["ASP.NET Core".into(), "Blazor".into(), "WebSockets".into()],
    })
    .add_project(Project {
      title: "Procedural Poetry Funhouse".into(),
      organisation: None,
      description: Some("Virtual Reality experience that aims to use the philosophy of play-based learning to change the way students perceive and experience poetry.".into()),
      tools_used: vec!["UnrealEngine".into(), "WebSockets".into(), "Vector Stores".into()],
    })
    .add_project(Project {
      title: "Local LLM RAG Application".into(),
      organisation: None,
      description: Some("Backend development for a RAG application using local models and vector stores".into()),
      tools_used: vec!["Local AI Models (Ollama)".into(), "Vector Stores".into(), "Qdrant".into(), "Flask".into()],
    })
    .add_project(Project {
      title: "VR Makerspace".into(),
      organisation: None,
      description: Some("Offers activities for students in the Online Master of Education, Literacy Education program and students in the Master of Educational Technology (MET) program to engage in multiplayer and distributed collaboration time.".into()),
      tools_used: vec!["UnrealEngine".into(), "Epic Online Services".into()],
    })
    .add_project(Project {
      title: "School District Phone Line Monitoring System".into(),
      organisation: Some("School District 36".into()),
      description: Some("Created a comprehensive phone line monitoring system for School District 36".into()),
      tools_used: vec!["StarTrinity".into(), "SIPp".into(), "Twilio".into(), "Bash Scripts".into()],
    })
    .add_project(Project {
      title: "UNICEF Bandwidth Monitor".into(),
      organisation: Some("UNICEF".into()),
      description: Some("Developed front-end graphs and monitoring system for UNICEF's worldwide sites".into()),
      tools_used: vec!["Plotly.js".into(), "JQuery".into(), "Flask".into(), "Jupyter".into()],
    })
}
