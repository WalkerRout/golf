interface FeedConfig {
  containerSelector: string;
  perPage?: number;
}

interface PostMeta {
  id: number;
  title: string;
  summary: string;
  source: string;
  published: string;
  link: string | null;
}

interface FeedPageResponse {
  posts: PostMeta[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}

interface FeedState {
  container: HTMLElement;
  perPage: number;
  currentPage: number;
  totalPages: number;
  total: number;
  pageCache: Map<number, FeedPageResponse>;
  activeCard: HTMLElement | null;
  savedScrollTop: number;
}

const PAGE_CACHE: Map<number, FeedPageResponse> = new Map();

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

async function fetchPage(page: number, perPage: number): Promise<FeedPageResponse> {
  const cached = PAGE_CACHE.get(page);
  if (cached) return cached;

  const response = await fetch(`/api/feed?page=${page}&per_page=${perPage}`);
  const data: FeedPageResponse = await response.json();
  PAGE_CACHE.set(page, data);
  return data;
}

function createPostCard(post: PostMeta): HTMLElement {
  const article = document.createElement('article');
  article.className = 'post-card';
  article.dataset.postId = String(post.id);
  article.dataset.link = post.link || '';

  article.innerHTML = `
    <div class="post-header">
      <span class="post-title">${escapeHtml(post.title)}</span>
      <svg class="expand-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <polyline points="9 18 15 12 9 6"></polyline>
      </svg>
    </div>
    <p class="post-summary">${escapeHtml(post.summary)}</p>
    <div class="post-footer">
      <span class="post-source">${escapeHtml(post.source)}</span>
      <span class="post-date">${escapeHtml(post.published)}</span>
    </div>
  `;

  return article;
}

function escapeHtml(text: string): string {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

function renderPaginationControls(state: FeedState): string {
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

  const pageButtons = pages.map(p => {
    if (p === '...') {
      return '<span class="pagination-ellipsis">...</span>';
    }
    const isActive = p === currentPage ? ' active' : '';
    return `<button class="pagination-page${isActive}" data-page="${p}">${p}</button>`;
  }).join('');

  return `
    <button class="pagination-prev" ${currentPage === 1 ? 'disabled' : ''} aria-label="Previous page">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <polyline points="15 18 9 12 15 6"></polyline>
      </svg>
    </button>
    <div class="pagination-pages">
      ${pageButtons}
    </div>
    <button class="pagination-next" ${currentPage === totalPages ? 'disabled' : ''} aria-label="Next page">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <polyline points="9 18 15 12 9 6"></polyline>
      </svg>
    </button>
  `;
}

function showSkeletons(container: HTMLElement): void {
  container.innerHTML = '';
  for (let i = 0; i < 6; i++) {
    const skeleton = document.createElement('article');
    skeleton.className = 'post-card skeleton';
    container.appendChild(skeleton);
  }
}

async function showPage(state: FeedState): Promise<void> {
  const data = await fetchPage(state.currentPage, state.perPage);

  state.totalPages = data.total_pages;
  state.total = data.total;

  // render post cards
  state.container.innerHTML = '';
  const fragment = document.createDocumentFragment();
  for (const post of data.posts) {
    const card = createPostCard(post);
    card.addEventListener('click', (e) => {
      e.stopPropagation();
      openDetail(state, card);
    });
    fragment.appendChild(card);
  }
  state.container.appendChild(fragment);

  if (data.posts.length === 0) {
    state.container.innerHTML = '<div class="empty-state"><p>No posts available.</p></div>';
  }

  setPageHash(state.currentPage);
  renderControls(state);
}

function renderControls(state: FeedState): void {
  if (state.totalPages <= 1) {
    // remove existing controls
    document.querySelectorAll('.pagination-controls').forEach(el => el.remove());
    return;
  }

  const html = renderPaginationControls(state);

  // ensure controls exist above and below the container
  let topControls = state.container.previousElementSibling as HTMLElement | null;
  if (!topControls || !topControls.classList.contains('pagination-controls')) {
    topControls = document.createElement('div');
    topControls.className = 'pagination-controls';
    state.container.parentElement?.insertBefore(topControls, state.container);
  }
  topControls.innerHTML = html;

  let bottomControls = state.container.nextElementSibling as HTMLElement | null;
  if (!bottomControls || !bottomControls.classList.contains('pagination-controls')) {
    bottomControls = document.createElement('div');
    bottomControls.className = 'pagination-controls';
    state.container.parentElement?.insertBefore(bottomControls, state.container.nextSibling);
  }
  bottomControls.innerHTML = html;

  // attach click handlers
  [topControls, bottomControls].forEach(controls => {
    controls.addEventListener('click', (e) => {
      const target = e.target as HTMLElement;
      const button = target.closest('button') as HTMLButtonElement | null;
      if (!button || button.disabled) return;

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
}

function goToPage(state: FeedState, page: number): void {
  const newPage = Math.max(1, Math.min(page, state.totalPages));
  if (newPage !== state.currentPage) {
    state.currentPage = newPage;
    showSkeletons(state.container);
    showPage(state);
    const scrollParent = state.container.closest('.posts-panel') || state.container;
    scrollParent.scrollTo({ top: 0, behavior: 'smooth' });
  }
}

function openDetail(state: FeedState, card: HTMLElement): void {
  const container = document.getElementById('feedContainer')!;
  const detailTitle = document.getElementById('detailTitle')!;
  const detailLink = document.getElementById('detailLink')! as HTMLAnchorElement;
  const detailFrame = document.getElementById('detailFrame')! as HTMLIFrameElement;
  const detailLoading = document.getElementById('detailLoading');

  const link = card.dataset.link;
  const title = card.querySelector('.post-title')!.textContent || '';

  detailTitle.textContent = title;

  if (link) {
    detailFrame.removeAttribute('srcdoc');
    detailFrame.src = link;
    detailLink.href = link;
    detailLink.style.display = 'flex';
    if (detailLoading) detailLoading.style.display = 'flex';
    detailFrame.onload = () => {
      if (detailLoading) detailLoading.style.display = 'none';
    };
  } else {
    detailFrame.removeAttribute('src');
    detailFrame.srcdoc = '<p style="padding: 2rem; font-family: sans-serif; color: #666;">No content available</p>';
    detailLink.style.display = 'none';
    if (detailLoading) detailLoading.style.display = 'none';
  }

  const wasOpen = container.classList.contains('detail-open');
  const postsPanel = document.querySelector('.posts-panel')!;

  if (!wasOpen) {
    state.savedScrollTop = postsPanel.scrollTop;
  }

  if (state.activeCard) state.activeCard.classList.remove('active');
  card.classList.add('active');
  state.activeCard = card;

  container.classList.add('detail-open');
  card.scrollIntoView({ block: 'center' });
}

function closeDetail(state: FeedState): void {
  const container = document.getElementById('feedContainer')!;
  const detailFrame = document.getElementById('detailFrame')! as HTMLIFrameElement;

  container.classList.remove('detail-open');
  if (state.activeCard) {
    state.activeCard.classList.remove('active');
    state.activeCard = null;
  }

  document.querySelector('.posts-panel')!.scrollTop = state.savedScrollTop;

  setTimeout(() => {
    if (!container.classList.contains('detail-open')) {
      detailFrame.removeAttribute('src');
      detailFrame.removeAttribute('srcdoc');
    }
  }, 300);
}

export function initFeed(config: FeedConfig): void {
  const { containerSelector, perPage = 24 } = config;

  const container = document.querySelector<HTMLElement>(containerSelector);
  if (!container) return;

  const state: FeedState = {
    container,
    perPage,
    currentPage: Math.max(1, getPageFromHash()),
    totalPages: 1,
    total: 0,
    pageCache: PAGE_CACHE,
    activeCard: null,
    savedScrollTop: 0,
  };

  // initial load
  showPage(state);

  // close detail panel
  const detailClose = document.getElementById('detailClose');
  if (detailClose) {
    detailClose.addEventListener('click', () => closeDetail(state));
  }

  // keyboard navigation
  document.addEventListener('keydown', (e) => {
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;

    const container = document.getElementById('feedContainer')!;

    if (e.key === 'Escape' && container.classList.contains('detail-open')) {
      closeDetail(state);
    } else if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') {
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
      showSkeletons(state.container);
      showPage(state);
    }
  });

  // clicking outside content closes detail
  document.querySelector('main')?.addEventListener('click', (e) => {
    const feedContainer = document.getElementById('feedContainer')!;
    if (feedContainer.classList.contains('detail-open') && !(e.target as HTMLElement).closest('.content')) {
      closeDetail(state);
    }
  });
}
