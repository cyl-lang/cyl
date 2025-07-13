// Cyl Documentation JavaScript
document.addEventListener('DOMContentLoaded', function() {
    console.log('Cyl Documentation loaded');
    
    // Hamburger Menu Functionality
    const hamburger = document.querySelector('.hamburger-menu');
    const navLinks = document.querySelector('.nav-links');
    
    if (hamburger && navLinks) {
        hamburger.addEventListener('click', function() {
            hamburger.classList.toggle('active');
            navLinks.classList.toggle('active');
        });
    }
    
    // Quick Navigation Expand/Collapse
    const expandToggles = document.querySelectorAll('.expand-toggle');
    
    expandToggles.forEach(toggle => {
        toggle.addEventListener('click', function() {
            const targetId = this.getAttribute('data-target');
            const subNav = document.getElementById(targetId);
            
            if (subNav) {
                const isVisible = subNav.style.display === 'block';
                subNav.style.display = isVisible ? 'none' : 'block';
                this.classList.toggle('expanded', !isVisible);
            }
        });
    });
    
    // Smooth scrolling for anchor links
    const anchorLinks = document.querySelectorAll('a[href^="#"]');
    
    anchorLinks.forEach(link => {
        link.addEventListener('click', function(e) {
            const href = this.getAttribute('href');
            if (href === '#') return;
            
            const targetElement = document.querySelector(href);
            if (targetElement) {
                e.preventDefault();
                targetElement.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
                
                // Close mobile menu if open
                if (navLinks && navLinks.classList.contains('active')) {
                    hamburger.classList.remove('active');
                    navLinks.classList.remove('active');
                }
            }
        });
    });
    
    // Syntax Block Modal Functionality (legacy support)
    const syntaxBlocks = document.querySelectorAll('[data-toggle^="syntax-"]');
    const overlays = document.querySelectorAll('.syntax-details-overlay');
    const closeButtons = document.querySelectorAll('.syntax-close-btn');
    
    // Open modal when clicking syntax block
    syntaxBlocks.forEach(block => {
        block.addEventListener('click', function() {
            const targetId = this.getAttribute('data-toggle');
            const overlay = document.getElementById(targetId);
            
            if (overlay) {
                // Close any open overlays first
                overlays.forEach(o => o.style.display = 'none');
                
                // Show the target overlay
                overlay.style.display = 'flex';
                document.body.style.overflow = 'hidden'; // Prevent background scroll
            }
        });
    });
    
    // Close modal when clicking close button
    closeButtons.forEach(button => {
        button.addEventListener('click', function() {
            const targetId = this.getAttribute('data-close');
            const overlay = document.getElementById(targetId);
            
            if (overlay) {
                overlay.style.display = 'none';
                document.body.style.overflow = 'auto'; // Restore scroll
            }
        });
    });
    
    // Close modal when clicking outside content
    overlays.forEach(overlay => {
        overlay.addEventListener('click', function(e) {
            if (e.target === this) {
                this.style.display = 'none';
                document.body.style.overflow = 'auto';
            }
        });
    });
    
    // Close modal with Escape key
    document.addEventListener('keydown', function(e) {
        if (e.key === 'Escape') {
            overlays.forEach(overlay => {
                if (overlay.style.display === 'flex') {
                    overlay.style.display = 'none';
                    document.body.style.overflow = 'auto';
                }
            });
            
            // Close mobile menu with Escape
            if (navLinks && navLinks.classList.contains('active')) {
                hamburger.classList.remove('active');
                navLinks.classList.remove('active');
            }
        }
    });
    
    // Feature card animations
    const features = document.querySelectorAll('.feature');
    
    // Intersection Observer for animations
    if ('IntersectionObserver' in window) {
        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.style.opacity = '1';
                    entry.target.style.transform = 'translateY(0)';
                }
            });
        }, {
            threshold: 0.1,
            rootMargin: '0px 0px -50px 0px'
        });
        
        features.forEach(feature => {
            feature.style.opacity = '0';
            feature.style.transform = 'translateY(20px)';
            feature.style.transition = 'opacity 0.6s ease, transform 0.6s ease';
            observer.observe(feature);
        });
    }
    
    // Highlight current section in navigation
    function highlightCurrentSection() {
        const sections = document.querySelectorAll('.syntax-category');
        const navLinks = document.querySelectorAll('.category-link');
        
        let currentSection = '';
        
        sections.forEach(section => {
            const rect = section.getBoundingClientRect();
            if (rect.top <= 100) {
                currentSection = section.id;
            }
        });
        
        navLinks.forEach(link => {
            const href = link.getAttribute('href');
            if (href === '#' + currentSection) {
                link.style.backgroundColor = '#f6f8fa';
                link.style.color = '#0969da';
            } else {
                link.style.backgroundColor = '';
                link.style.color = '';
            }
        });
    }
    
    // Throttled scroll handler
    let scrollTimeout;
    window.addEventListener('scroll', function() {
        if (scrollTimeout) {
            clearTimeout(scrollTimeout);
        }
        scrollTimeout = setTimeout(highlightCurrentSection, 100);
    });
    
    // Initial highlight
    highlightCurrentSection();
    
    // Copy to clipboard functionality for code blocks
    const codeBlocks = document.querySelectorAll('.syntax-code pre, .example-code pre, .backend-usage');
    
    codeBlocks.forEach(block => {
        // Add copy button
        const copyButton = document.createElement('button');
        copyButton.textContent = 'Copy';
        copyButton.className = 'copy-button';
        copyButton.style.cssText = `
            position: absolute;
            top: 8px;
            right: 8px;
            background: #0969da;
            color: white;
            border: none;
            padding: 4px 8px;
            border-radius: 4px;
            font-size: 12px;
            cursor: pointer;
            opacity: 0;
            transition: opacity 0.2s;
            z-index: 10;
        `;
        
        const container = block.parentElement;
        container.style.position = 'relative';
        container.appendChild(copyButton);
        
        // Show/hide copy button on hover
        container.addEventListener('mouseenter', () => {
            copyButton.style.opacity = '1';
        });
        
        container.addEventListener('mouseleave', () => {
            copyButton.style.opacity = '0';
        });
        
        // Copy functionality
        copyButton.addEventListener('click', async () => {
            try {
                await navigator.clipboard.writeText(block.textContent);
                copyButton.textContent = 'Copied!';
                copyButton.style.background = '#28a745';
                
                setTimeout(() => {
                    copyButton.textContent = 'Copy';
                    copyButton.style.background = '#0969da';
                }, 2000);
            } catch (err) {
                console.error('Failed to copy text: ', err);
                copyButton.textContent = 'Failed';
                copyButton.style.background = '#dc3545';
                
                setTimeout(() => {
                    copyButton.textContent = 'Copy';
                    copyButton.style.background = '#0969da';
                }, 2000);
            }
        });
    });
    
    // Search functionality (if search input exists)
    const searchInput = document.querySelector('#search-input');
    if (searchInput) {
        let searchTimeout;
        
        searchInput.addEventListener('input', function() {
            clearTimeout(searchTimeout);
            searchTimeout = setTimeout(() => {
                performSearch(this.value);
            }, 300);
        });
        
        function performSearch(query) {
            const searchResults = document.querySelector('#search-results');
            if (!searchResults) return;
            
            if (!query.trim()) {
                searchResults.innerHTML = '';
                searchResults.style.display = 'none';
                return;
            }
            
            // Simple search through syntax blocks
            const syntaxBlocks = document.querySelectorAll('.syntax-block');
            const results = [];
            
            syntaxBlocks.forEach(block => {
                const title = block.querySelector('.syntax-title').textContent;
                const description = block.querySelector('.syntax-description').textContent;
                const code = block.querySelector('.syntax-code').textContent;
                
                const searchText = (title + ' ' + description + ' ' + code).toLowerCase();
                if (searchText.includes(query.toLowerCase())) {
                    results.push({
                        title: title,
                        description: description,
                        element: block
                    });
                }
            });
            
            // Display results
            if (results.length > 0) {
                searchResults.innerHTML = results.map(result => `
                    <div class="search-result" data-target="${result.element.id}">
                        <h4>${result.title}</h4>
                        <p>${result.description}</p>
                    </div>
                `).join('');
                
                // Add click handlers for search results
                document.querySelectorAll('.search-result').forEach(result => {
                    result.addEventListener('click', function() {
                        const targetId = this.getAttribute('data-target');
                        const targetElement = document.getElementById(targetId);
                        if (targetElement) {
                            targetElement.scrollIntoView({ behavior: 'smooth' });
                            searchInput.value = '';
                            searchResults.innerHTML = '';
                            searchResults.style.display = 'none';
                        }
                    });
                });
                
                searchResults.style.display = 'block';
            } else {
                searchResults.innerHTML = '<div class="no-results">No results found</div>';
                searchResults.style.display = 'block';
            }
        }
    }
    
    // Table of contents generation for long pages
    function generateTableOfContents() {
        const tocContainer = document.querySelector('#table-of-contents');
        if (!tocContainer) return;
        
        const headings = document.querySelectorAll('h2, h3');
        if (headings.length === 0) return;
        
        const tocList = document.createElement('ul');
        tocList.className = 'toc-list';
        
        headings.forEach((heading, index) => {
            if (!heading.id) {
                heading.id = `heading-${index}`;
            }
            
            const tocItem = document.createElement('li');
            tocItem.className = `toc-item toc-${heading.tagName.toLowerCase()}`;
            
            const tocLink = document.createElement('a');
            tocLink.href = `#${heading.id}`;
            tocLink.textContent = heading.textContent;
            tocLink.className = 'toc-link';
            
            tocItem.appendChild(tocLink);
            tocList.appendChild(tocItem);
        });
        
        tocContainer.appendChild(tocList);
    }
    
    generateTableOfContents();
    
    console.log('Cyl Documentation interactions initialized');
});
