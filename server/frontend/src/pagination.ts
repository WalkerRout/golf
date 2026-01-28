interface PaginationConfig {
  containerSelector: string;
  itemSelector: string;
  perPage?: number;
  controlsPosition?: 'top' | 'bottom' | 'both';
}

interface PaginationState {
  currentPage: number;
  totalPages: number;
  items: HTMLElement[];
  container: HTMLElement;
  controls: HTMLElement[];
  perPage: number;
}

const DEFAULT_PER_PAGE = 20;

function getPageFromHash(): number {
  const match = window.location.hash.match(/page=(\d+)/);
  return match ? Math.max(1, parseInt(match[1], 10)) : 1;
}

function setPageHash(page: number): void {
  const newHash = page === 1 ? '' : `#page=${page}`;
  if (window.location.hash !== newHash) {
    history.replaceState(null, '', newHash || window.location.pathname);
  }
}

function createControlsElement(): HTMLElement {
  const controls = document.createElement('div');
  controls.className = 'pagination-controls';
  return controls;
}

function renderPageNumbers(state: PaginationState): string {
  const { currentPage, totalPages } = state;
  const pages: (number | string)[] = [];

  if (totalPages <= 7) {
    for (let i = 1; i <= totalPages; i++) pages.push(i);
  } else {
    pages.push(1);
    if (currentPage > 3) pages.push('...');

    const start = Math.max(2, currentPage - 1);
    const end = Math.min(totalPages - 1, currentPage + 1);

    for (let i = start; i <= end; i++) pages.push(i);

    if (currentPage < totalPages - 2) pages.push('...');
    pages.push(totalPages);
  }

  return pages.map(p => {
    if (p === '...') {
      return '<span class="pagination-ellipsis">...</span>';
    }
    const isActive = p === currentPage ? ' active' : '';
    return `<button class="pagination-page${isActive}" data-page="${p}">${p}</button>`;
  }).join('');
}

function renderControls(state: PaginationState): void {
  const { currentPage, totalPages } = state;

  const html = `
    <button class="pagination-prev" ${currentPage === 1 ? 'disabled' : ''} aria-label="Previous page">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <polyline points="15 18 9 12 15 6"></polyline>
      </svg>
    </button>
    <div class="pagination-pages">
      ${renderPageNumbers(state)}
    </div>
    <button class="pagination-next" ${currentPage === totalPages ? 'disabled' : ''} aria-label="Next page">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <polyline points="9 18 15 12 9 6"></polyline>
      </svg>
    </button>
  `;

  state.controls.forEach(el => {
    el.innerHTML = html;
  });
}

function showPage(state: PaginationState): void {
  const { currentPage, items, container, perPage } = state;
  const start = (currentPage - 1) * perPage;
  const end = Math.min(start + perPage, items.length);

  // remove current items from DOM (but keep stored in items array)
  const currentChildren = Array.from(container.querySelectorAll<HTMLElement>(`:scope > :not(template)`));
  currentChildren.forEach(child => {
    if (!child.matches('template')) {
      child.remove();
    }
  });

  // add only current page items to DOM
  const fragment = document.createDocumentFragment();
  for (let i = start; i < end; i++) {
    fragment.appendChild(items[i]);
  }
  container.appendChild(fragment);

  setPageHash(currentPage);
  renderControls(state);
}

function goToPage(state: PaginationState, page: number): void {
  const newPage = Math.max(1, Math.min(page, state.totalPages));
  if (newPage !== state.currentPage) {
    state.currentPage = newPage;
    showPage(state);
    // scroll the scrollable parent to top
    const scrollParent = state.container.closest('.posts-panel, .congeries') || state.container;
    scrollParent.scrollTo({ top: 0, behavior: 'smooth' });
  }
}

function attachEventListeners(state: PaginationState): void {
  state.controls.forEach(controls => {
    controls.addEventListener('click', (e) => {
      const target = e.target as HTMLElement;
      const button = target.closest('button');
      if (!button || (button as HTMLButtonElement).disabled) return;

      if (button.classList.contains('pagination-prev')) {
        goToPage(state, state.currentPage - 1);
      } else if (button.classList.contains('pagination-next')) {
        goToPage(state, state.currentPage + 1);
      } else if (button.classList.contains('pagination-page')) {
        const page = parseInt(button.dataset.page || '1', 10);
        goToPage(state, page);
      }
    });
  });

  // keyboard navigation
  document.addEventListener('keydown', (e) => {
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;

    if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') {
      e.preventDefault();
      goToPage(state, state.currentPage - 1);
    } else if (e.key === 'ArrowRight' || e.key === 'ArrowDown') {
      e.preventDefault();
      goToPage(state, state.currentPage + 1);
    }
  });

  // handle browser back/forward
  window.addEventListener('hashchange', () => {
    const page = getPageFromHash();
    if (page !== state.currentPage) {
      state.currentPage = page;
      showPage(state);
    }
  });
}

export function initPagination(config: PaginationConfig): void {
  const { containerSelector, itemSelector, perPage = DEFAULT_PER_PAGE, controlsPosition = 'both' } = config;

  const container = document.querySelector<HTMLElement>(containerSelector);
  if (!container) return;

  // collect all items and remove them from DOM (store in memory)
  const items = Array.from(container.querySelectorAll<HTMLElement>(itemSelector));
  if (items.length === 0) return;

  const totalPages = Math.ceil(items.length / perPage);
  if (totalPages <= 1) return; // no pagination needed

  // remove all items from DOM (theyre stored in items array)
  items.forEach(item => item.remove());

  const currentPage = Math.min(getPageFromHash(), totalPages);
  const controls: HTMLElement[] = [];

  // insert controls
  if (controlsPosition === 'top' || controlsPosition === 'both') {
    const topControls = createControlsElement();
    container.parentElement?.insertBefore(topControls, container);
    controls.push(topControls);
  }

  if (controlsPosition === 'bottom' || controlsPosition === 'both') {
    const bottomControls = createControlsElement();
    container.parentElement?.insertBefore(bottomControls, container.nextSibling);
    controls.push(bottomControls);
  }

  const state: PaginationState = {
    currentPage,
    totalPages,
    items,
    container,
    controls,
    perPage,
  };

  showPage(state);
  attachEventListeners(state);
}
