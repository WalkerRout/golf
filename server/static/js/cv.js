document.addEventListener('DOMContentLoaded', () => {
  const tabContainer = document.querySelector('.cv-menu');
  
  tabContainer.addEventListener('click', (e) => {
    const tab = e.target.closest('.cv-tab');
    if (!tab) return;

    // remove active class from all tabs and sections
    document.querySelectorAll('.cv-tab').forEach(t => t.classList.remove('active'));
    document.querySelectorAll('.cv-section').forEach(s => s.classList.remove('active'));

    // add active class to clicked tab and corresponding section
    tab.classList.add('active');
    document.getElementById(`${tab.dataset.tab}-tab`).classList.add('active');
  });
});