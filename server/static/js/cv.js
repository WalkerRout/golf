document.addEventListener('DOMContentLoaded', () => {
  const tabContainer = document.querySelector('.cv-menu');
  const tabs = Array.from(tabContainer.querySelectorAll('.cv-tab'));
  const sections = Array.from(document.querySelectorAll('.cv-section'));

  // map tab data attributes to corresponding section elements
  const sectionMap = new Map(
    sections.map(section => [section.id.replace('-tab', ''), section])
  );

  let activeTab = tabContainer.querySelector('.cv-tab.active');
  let activeSection = document.querySelector('.cv-section.active');

  tabContainer.addEventListener('click', (e) => {
    const tab = e.target.closest('.cv-tab');
    if (!tab || tab === activeTab) return;

    const tabKey = tab.dataset.tab;
    const section = sectionMap.get(tabKey);
    if (!section) return;

    // Only change classes if different tab clicked
    activeTab.classList.remove('active');
    activeSection.classList.remove('active');

    tab.classList.add('active');
    section.classList.add('active');

    activeTab = tab;
    activeSection = section;
  });
});
