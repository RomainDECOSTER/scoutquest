// Documentation-specific JavaScript functionality

document.addEventListener('DOMContentLoaded', function() {
    // Clean any existing malformed HTML first
    cleanMalformedHTML();

    // Enhanced tab functionality for documentation
    initDocumentationTabs();

    // Copy code functionality
    initCodeCopyButtons();

    // Smooth scrolling for documentation links
    initDocumentationScrolling();

    // Table of contents generation
    generateTableOfContents();

    // Search functionality
    initDocumentationSearch();

    // Code syntax highlighting
    initSyntaxHighlighting();
});

// Clean malformed HTML elements that might be left from previous highlighting
function cleanMalformedHTML() {
    const codeBlocks = document.querySelectorAll('pre code');

    codeBlocks.forEach(block => {
        // Remove any existing highlighting classes first
        block.classList.remove('highlighted');

        // Get clean text content and reset
        const cleanText = block.textContent;
        block.innerHTML = '';
        block.textContent = cleanText;
    });
}

// Enhanced tab functionality for documentation pages
function initDocumentationTabs() {
    const tabContainers = document.querySelectorAll('.installation-tabs, .docs-tabs');

    tabContainers.forEach(container => {
        const tabButtons = container.querySelectorAll('.tab-btn');
        const tabContents = container.querySelectorAll('.tab-content');

        tabButtons.forEach(button => {
            button.addEventListener('click', function() {
                const targetTab = this.getAttribute('data-tab');

                // Remove active class from all buttons and contents in this container
                tabButtons.forEach(btn => btn.classList.remove('active'));
                tabContents.forEach(content => content.classList.remove('active'));

                // Add active class to clicked button and corresponding content
                this.classList.add('active');
                const targetContent = container.querySelector(`#${targetTab}`);
                if (targetContent) {
                    targetContent.classList.add('active');
                }

                // Save tab selection to localStorage
                localStorage.setItem(`docs-tab-${container.className}`, targetTab);
            });
        });

        // Restore saved tab selection
        const savedTab = localStorage.getItem(`docs-tab-${container.className}`);
        if (savedTab) {
            const savedButton = container.querySelector(`[data-tab="${savedTab}"]`);
            if (savedButton) {
                savedButton.click();
            }
        }
    });
}

// Copy code functionality with enhanced feedback
function initCodeCopyButtons() {
    const codeBlocks = document.querySelectorAll('.code-block pre code, pre code');

    codeBlocks.forEach(block => {
        const pre = block.closest('pre');
        if (!pre || pre.querySelector('.copy-btn')) return; // Avoid duplicates

        const button = document.createElement('button');
        button.className = 'copy-btn';
        button.innerHTML = `
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                <path d="m5 15-4-4 4-4"></path>
            </svg>
            Copy
        `;

        button.addEventListener('click', async () => {
            try {
                await navigator.clipboard.writeText(block.textContent);

                // Success feedback
                button.innerHTML = `
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <polyline points="20,6 9,17 4,12"></polyline>
                    </svg>
                    Copied!
                `;
                button.style.background = 'rgba(16, 185, 129, 0.2)';
                button.style.borderColor = 'rgba(16, 185, 129, 0.4)';

                // Reset after delay
                setTimeout(() => {
                    button.innerHTML = `
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                            <path d="m5 15-4-4 4-4"></path>
                        </svg>
                        Copy
                    `;
                    button.style.background = 'rgba(255, 255, 255, 0.1)';
                    button.style.borderColor = 'rgba(255, 255, 255, 0.2)';
                }, 2000);
            } catch (err) {
                console.error('Failed to copy code:', err);

                // Error feedback
                button.innerHTML = `
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <circle cx="12" cy="12" r="10"></circle>
                        <line x1="15" y1="9" x2="9" y2="15"></line>
                        <line x1="9" y1="9" x2="15" y2="15"></line>
                    </svg>
                    Failed
                `;
                button.style.background = 'rgba(239, 68, 68, 0.2)';
                button.style.borderColor = 'rgba(239, 68, 68, 0.4)';

                setTimeout(() => {
                    button.innerHTML = `
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                            <path d="m5 15-4-4 4-4"></path>
                        </svg>
                        Copy
                    `;
                    button.style.background = 'rgba(255, 255, 255, 0.1)';
                    button.style.borderColor = 'rgba(255, 255, 255, 0.2)';
                }, 2000);
            }
        });

        // Make pre relative positioned for absolute button
        pre.style.position = 'relative';
        pre.appendChild(button);
    });
}

// Enhanced smooth scrolling for documentation
function initDocumentationScrolling() {
    const links = document.querySelectorAll('a[href^="#"], .docs-nav__link[href*="#"]');

    links.forEach(link => {
        link.addEventListener('click', function(e) {
            const href = this.getAttribute('href');
            if (!href.includes('#')) return;

            e.preventDefault();

            const targetId = href.split('#')[1];
            const targetElement = document.getElementById(targetId);

            if (targetElement) {
                const headerHeight = document.querySelector('.header')?.offsetHeight || 0;
                const targetPosition = targetElement.offsetTop - headerHeight - 20;

                window.scrollTo({
                    top: targetPosition,
                    behavior: 'smooth'
                });

                // Update URL without jumping
                history.pushState(null, null, href);
            }
        });
    });
}

// Generate table of contents for long documentation pages
function generateTableOfContents() {
    const tocContainer = document.getElementById('table-of-contents');
    if (!tocContainer) return;

    const headings = document.querySelectorAll('.docs-content h2, .docs-content h3');
    if (headings.length < 3) return; // Only generate TOC for longer pages

    const tocList = document.createElement('ul');
    tocList.className = 'toc-list';

    headings.forEach((heading, index) => {
        // Generate ID if not present
        if (!heading.id) {
            heading.id = `heading-${index}`;
        }

        const listItem = document.createElement('li');
        const link = document.createElement('a');

        link.href = `#${heading.id}`;
        link.textContent = heading.textContent;
        link.className = `toc-link toc-${heading.tagName.toLowerCase()}`;

        listItem.appendChild(link);
        tocList.appendChild(listItem);
    });

    tocContainer.appendChild(tocList);

    // Highlight current section in TOC
    const tocLinks = tocContainer.querySelectorAll('.toc-link');
    const observer = new IntersectionObserver(
        (entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    const id = entry.target.getAttribute('id');

                    // Remove active class from all TOC links
                    tocLinks.forEach(link => link.classList.remove('active'));

                    // Add active class to current section
                    const activeLink = tocContainer.querySelector(`[href="#${id}"]`);
                    if (activeLink) {
                        activeLink.classList.add('active');
                    }
                }
            });
        },
        {
            threshold: 0.5,
            rootMargin: '-100px 0px -80% 0px'
        }
    );

    headings.forEach(heading => observer.observe(heading));
}

// Documentation search functionality
function initDocumentationSearch() {
    const searchInput = document.getElementById('docs-search');
    const searchResults = document.getElementById('search-results');

    if (!searchInput || !searchResults) return;

    let searchIndex = [];
    let searchTimeout;

    // Build search index
    function buildSearchIndex() {
        const content = document.querySelector('.docs-content');
        if (!content) return;

        const sections = content.querySelectorAll('section, .docs-section');

        sections.forEach((section, index) => {
            const title = section.querySelector('h2, h3, h4')?.textContent || '';
            const text = section.textContent || '';
            const id = section.id || `section-${index}`;

            searchIndex.push({
                id,
                title,
                text: text.toLowerCase(),
                element: section
            });
        });
    }

    // Perform search
    function performSearch(query) {
        const results = searchIndex.filter(item => {
            return item.text.includes(query.toLowerCase()) ||
                   item.title.toLowerCase().includes(query.toLowerCase());
        });

        displaySearchResults(results, query);
    }

    // Display search results
    function displaySearchResults(results, query) {
        if (results.length === 0) {
            searchResults.innerHTML = `
                <div class="search-no-results">
                    <p>No results found for "${query}"</p>
                </div>
            `;
        } else {
            const resultsHTML = results.map(result => {
                const excerpt = getExcerpt(result.text, query);
                return `
                    <div class="search-result" data-id="${result.id}">
                        <h4 class="search-result-title">${result.title}</h4>
                        <p class="search-result-excerpt">${excerpt}</p>
                    </div>
                `;
            }).join('');

            searchResults.innerHTML = resultsHTML;

            // Add click handlers for search results
            searchResults.querySelectorAll('.search-result').forEach(result => {
                result.addEventListener('click', () => {
                    const id = result.getAttribute('data-id');
                    const element = document.getElementById(id);
                    if (element) {
                        element.scrollIntoView({ behavior: 'smooth' });
                        searchResults.style.display = 'none';
                        searchInput.value = '';
                    }
                });
            });
        }

        searchResults.style.display = 'block';
    }

    // Get excerpt with highlighted query
    function getExcerpt(text, query, maxLength = 150) {
        const lowerText = text.toLowerCase();
        const lowerQuery = query.toLowerCase();
        const index = lowerText.indexOf(lowerQuery);

        if (index === -1) {
            return text.substring(0, maxLength) + (text.length > maxLength ? '...' : '');
        }

        const start = Math.max(0, index - 50);
        const end = Math.min(text.length, index + query.length + 50);
        let excerpt = text.substring(start, end);

        if (start > 0) excerpt = '...' + excerpt;
        if (end < text.length) excerpt = excerpt + '...';

        // Highlight query in excerpt
        const regex = new RegExp(`(${query})`, 'gi');
        excerpt = excerpt.replace(regex, '<strong>$1</strong>');

        return excerpt;
    }

    // Search input handler
    searchInput.addEventListener('input', function() {
        clearTimeout(searchTimeout);
        const query = this.value.trim();

        if (query.length < 2) {
            searchResults.innerHTML = '';
            searchResults.style.display = 'none';
            return;
        }

        searchTimeout = setTimeout(() => {
            performSearch(query);
        }, 300);
    });

    // Hide search results when clicking outside
    document.addEventListener('click', function(e) {
        if (!searchInput.contains(e.target) && !searchResults.contains(e.target)) {
            searchResults.style.display = 'none';
        }
    });

    // Build search index on load
    buildSearchIndex();
}

// Basic syntax highlighting for code blocks
function initSyntaxHighlighting() {
    // Utilisation de Prism.js pour le syntax highlighting
    // Prism.js est chargé via CDN dans les pages HTML

    const codeBlocks = document.querySelectorAll('pre code');

    codeBlocks.forEach(block => {
        // Nettoyer tout HTML malformé existant
        if (block.innerHTML.includes('class="syntax-')) {
            const cleanText = block.textContent;
            block.innerHTML = '';
            block.textContent = cleanText;
        }

        // Détecter le langage et appliquer les classes Prism
        const pre = block.closest('pre');
        const text = block.textContent.toLowerCase();

        // Détection automatique du langage
        if (text.includes('const ') || text.includes('function ') || text.includes('require(') || text.includes('import ')) {
            block.className = 'language-javascript';
        } else if (text.includes('fn ') || text.includes('use ') || text.includes('impl ') || text.includes('struct ')) {
            block.className = 'language-rust';
        } else if (text.includes('apiversion:') || text.includes('kind:') || text.includes('metadata:')) {
            block.className = 'language-yaml';
        } else if (text.includes('#!/bin/bash') || text.includes('curl ') || text.includes('docker ')) {
            block.className = 'language-bash';
        } else if (text.includes('select ') || text.includes('create table') || text.includes('insert into')) {
            block.className = 'language-sql';
        } else if (text.includes('[server]') || text.includes('[database]') || text.includes('[security]')) {
            block.className = 'language-toml';
        } else if (text.includes('{') && text.includes('"')) {
            block.className = 'language-json';
        } else {
            block.className = 'language-none';
        }

        // Marquer comme traité
        block.classList.add('highlighted');
    });

    // Réappliquer Prism.js si disponible
    if (typeof Prism !== 'undefined') {
        Prism.highlightAll();
    }
}

// Add CSS for syntax highlighting (now in docs.css)
function addSyntaxHighlightingStyles() {
    // Styles are now included in docs.css
    // This function is kept for backward compatibility
    return;
}

// Keyboard shortcuts for documentation
document.addEventListener('keydown', function(e) {
    // Ctrl/Cmd + K to focus search
    if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault();
        const searchInput = document.getElementById('docs-search');
        if (searchInput) {
            searchInput.focus();
        }
    }

    // Escape to close search results
    if (e.key === 'Escape') {
        const searchResults = document.getElementById('search-results');
        if (searchResults) {
            searchResults.style.display = 'none';
        }
    }
});

// Print styles optimization
function optimizeForPrint() {
    const mediaQuery = window.matchMedia('print');

    mediaQuery.addListener(function(mq) {
        if (mq.matches) {
            // Hide navigation and other non-content elements when printing
            const elementsToHide = document.querySelectorAll('.docs-sidebar, .header, .copy-btn, .search-results');
            elementsToHide.forEach(el => el.style.display = 'none');

            // Expand all code blocks
            const codeBlocks = document.querySelectorAll('.code-block');
            codeBlocks.forEach(block => {
                block.style.pageBreakInside = 'avoid';
                block.style.breakInside = 'avoid';
            });
        }
    });
}

// Initialize print optimization
document.addEventListener('DOMContentLoaded', optimizeForPrint);
