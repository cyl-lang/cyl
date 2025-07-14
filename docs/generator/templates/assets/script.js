// Documentation Site Interactivity
document.addEventListener("DOMContentLoaded", function () {
  console.log("Cyl Documentation loaded");

  // Mobile hamburger menu with enhanced functionality
  const hamburgerMenu = document.querySelector(".hamburger-menu");
  const navLinks = document.querySelector(".nav-links");

  if (hamburgerMenu && navLinks) {
    hamburgerMenu.addEventListener("click", function () {
      hamburgerMenu.classList.toggle("active");
      navLinks.classList.toggle("active");
    });

    // Close menu when clicking outside
    document.addEventListener("click", function (event) {
      if (
        !hamburgerMenu.contains(event.target) &&
        !navLinks.contains(event.target)
      ) {
        hamburgerMenu.classList.remove("active");
        navLinks.classList.remove("active");
      }
    });

    // Close menu when clicking on a link
    navLinks.querySelectorAll("a").forEach((link) => {
      link.addEventListener("click", function () {
        hamburgerMenu.classList.remove("active");
        navLinks.classList.remove("active");
      });
    });
  }

  // Quick Navigation Expand/Collapse
  const expandToggles = document.querySelectorAll(".expand-toggle");

  expandToggles.forEach((toggle) => {
    toggle.addEventListener("click", function () {
      const targetId = this.getAttribute("data-target");
      const subNav = document.getElementById(targetId);

      if (subNav) {
        const isVisible = subNav.style.display === "block";
        subNav.style.display = isVisible ? "none" : "block";
        this.classList.toggle("expanded", !isVisible);
      }
    });
  });

  // Smooth scrolling for anchor links
  const anchorLinks = document.querySelectorAll('a[href^="#"]');

  anchorLinks.forEach((link) => {
    link.addEventListener("click", function (e) {
      const href = this.getAttribute("href");
      if (href === "#") return;

      const targetElement = document.querySelector(href);
      if (targetElement) {
        e.preventDefault();
        targetElement.scrollIntoView({
          behavior: "smooth",
          block: "start",
        });

        // Close mobile menu if open
        if (navLinks && navLinks.classList.contains("active")) {
          hamburger.classList.remove("active");
          navLinks.classList.remove("active");
        }
      }
    });
  });

  // Syntax Block Modal Functionality (legacy support)
  const syntaxBlocks = document.querySelectorAll('[data-toggle^="syntax-"]');
  const overlays = document.querySelectorAll(".syntax-details-overlay");
  const closeButtons = document.querySelectorAll(".syntax-close-btn");

  // Open modal when clicking syntax block
  syntaxBlocks.forEach((block) => {
    block.addEventListener("click", function () {
      const targetId = this.getAttribute("data-toggle");
      const overlay = document.getElementById(targetId);

      if (overlay) {
        // Close any open overlays first
        overlays.forEach((o) => (o.style.display = "none"));

        // Show the target overlay
        overlay.style.display = "flex";
        document.body.style.overflow = "hidden"; // Prevent background scroll
      }
    });
  });

  // Close modal when clicking close button
  closeButtons.forEach((button) => {
    button.addEventListener("click", function () {
      const targetId = this.getAttribute("data-close");
      const overlay = document.getElementById(targetId);

      if (overlay) {
        overlay.style.display = "none";
        document.body.style.overflow = "auto"; // Restore scroll
      }
    });
  });

  // Close modal when clicking outside content
  overlays.forEach((overlay) => {
    overlay.addEventListener("click", function (e) {
      if (e.target === this) {
        this.style.display = "none";
        document.body.style.overflow = "auto";
      }
    });
  });

  // Close modal with Escape key
  document.addEventListener("keydown", function (e) {
    if (e.key === "Escape") {
      overlays.forEach((overlay) => {
        if (overlay.style.display === "flex") {
          overlay.style.display = "none";
          document.body.style.overflow = "auto";
        }
      });

      // Close mobile menu with Escape
      if (navLinks && navLinks.classList.contains("active")) {
        hamburger.classList.remove("active");
        navLinks.classList.remove("active");
      }
    }
  });

  // Header scroll effect with dynamic styling
  const header = document.querySelector(".header");
  if (header) {
    let lastScrollY = window.scrollY;

    window.addEventListener("scroll", function () {
      const currentScrollY = window.scrollY;

      if (currentScrollY > 100) {
        header.style.boxShadow = "0 2px 20px rgba(0, 0, 0, 0.1)";
        header.style.background = "rgba(255, 255, 255, 0.98)";
      } else {
        header.style.boxShadow = "none";
        header.style.background = "rgba(255, 255, 255, 0.95)";
      }

      lastScrollY = currentScrollY;
    });
  }

  // Animate features on scroll with enhanced effects
  const features = document.querySelectorAll(".feature");
  const observerOptions = {
    threshold: 0.1,
    rootMargin: "0px 0px -50px 0px",
  };

  const featureObserver = new IntersectionObserver(function (entries) {
    entries.forEach((entry, index) => {
      if (entry.isIntersecting) {
        setTimeout(() => {
          entry.target.style.opacity = "1";
          entry.target.style.transform = "translateY(0)";
        }, index * 150); // Staggered animation
      }
    });
  }, observerOptions);

  features.forEach((feature) => {
    feature.style.opacity = "0";
    feature.style.transform = "translateY(30px)";
    feature.style.transition = "opacity 0.6s ease, transform 0.6s ease";
    featureObserver.observe(feature);
  });

  // Enhanced button hover effects
  const buttons = document.querySelectorAll(".btn-primary, .btn-secondary");
  buttons.forEach((button) => {
    button.addEventListener("mouseenter", function () {
      this.style.transform = "translateY(-2px) scale(1.02)";
      createSparkleEffect(this);
    });

    button.addEventListener("mouseleave", function () {
      this.style.transform = "translateY(0) scale(1)";
    });
  });

  // Sparkle effect for buttons
  function createSparkleEffect(element) {
    const sparkle = document.createElement("div");
    sparkle.className = "sparkle";
    sparkle.style.cssText = `
            position: absolute;
            width: 4px;
            height: 4px;
            background: rgba(255, 255, 255, 0.8);
            border-radius: 50%;
            pointer-events: none;
            animation: sparkle 0.8s ease-out forwards;
            top: ${Math.random() * 100}%;
            left: ${Math.random() * 100}%;
            z-index: 1;
        `;

    // Add sparkle animation if not already present
    if (!document.querySelector("#sparkle-styles")) {
      const sparkleStyle = document.createElement("style");
      sparkleStyle.id = "sparkle-styles";
      sparkleStyle.textContent = `
                @keyframes sparkle {
                    0% {
                        opacity: 0;
                        transform: scale(0) rotate(0deg);
                    }
                    50% {
                        opacity: 1;
                        transform: scale(1) rotate(180deg);
                    }
                    100% {
                        opacity: 0;
                        transform: scale(0) rotate(360deg);
                    }
                }
            `;
      document.head.appendChild(sparkleStyle);
    }

    element.style.position = "relative";
    element.appendChild(sparkle);

    setTimeout(() => {
      if (sparkle && sparkle.parentNode) {
        sparkle.parentNode.removeChild(sparkle);
      }
    }, 800);
  }

  // Enhanced copy functionality for code blocks
  document.querySelectorAll("pre code").forEach((codeBlock) => {
    const button = document.createElement("button");
    button.className = "copy-button";
    button.innerHTML = `
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
            </svg>
        `;
    button.style.cssText = `
            position: absolute;
            top: 12px;
            right: 12px;
            background: linear-gradient(135deg, #667eea, #764ba2);
            color: white;
            border: none;
            padding: 8px;
            border-radius: 6px;
            cursor: pointer;
            opacity: 0;
            transition: all 0.2s ease;
            display: flex;
            align-items: center;
            justify-content: center;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
        `;

    const pre = codeBlock.parentNode;
    pre.style.position = "relative";
    pre.appendChild(button);

    pre.addEventListener("mouseenter", () => {
      button.style.opacity = "1";
      button.style.transform = "scale(1)";
    });
    pre.addEventListener("mouseleave", () => {
      button.style.opacity = "0";
      button.style.transform = "scale(0.9)";
    });

    button.addEventListener("click", async () => {
      try {
        await navigator.clipboard.writeText(codeBlock.textContent);
        button.innerHTML = `
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <polyline points="20,6 9,17 4,12"></polyline>
                    </svg>
                `;
        button.style.background = "#059669";
        setTimeout(() => {
          button.innerHTML = `
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
                            <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
                        </svg>
                    `;
          button.style.background = "linear-gradient(135deg, #667eea, #764ba2)";
        }, 2000);
      } catch (err) {
        console.error("Failed to copy: ", err);
      }
    });
  });

  // Active navigation highlighting
  const navLinksElements = document.querySelectorAll(".nav-links a");
  const currentPath = window.location.pathname;

  navLinksElements.forEach((link) => {
    const linkPath = link.getAttribute("href");
    if (
      linkPath === currentPath ||
      (currentPath.includes("index.html") && linkPath === "/") ||
      (currentPath === "/" && linkPath === "index.html")
    ) {
      link.classList.add("active");
    }
  });

  // Parallax effect for hero section
  const hero = document.querySelector(".hero");
  if (hero) {
    window.addEventListener("scroll", () => {
      const scrolled = window.pageYOffset;
      const rate = scrolled * -0.5;
      hero.style.transform = `translateY(${rate}px)`;
    });
  }

  // Add floating animation to stats
  const stats = document.querySelectorAll(".stat");
  stats.forEach((stat, index) => {
    stat.style.animationDelay = `${index * 0.2}s`;
    stat.style.animation = "float 3s ease-in-out infinite";
  });

  // Add float animation if not present
  if (!document.querySelector("#float-styles")) {
    const floatStyle = document.createElement("style");
    floatStyle.id = "float-styles";
    floatStyle.textContent = `
            @keyframes float {
                0%, 100% {
                    transform: translateY(0px);
                }
                50% {
                    transform: translateY(-10px);
                }
            }
        `;
    document.head.appendChild(floatStyle);
  }

  // Table of contents generation for long pages
  function generateTableOfContents() {
    const tocContainer = document.querySelector("#table-of-contents");
    if (!tocContainer) return;

    const headings = document.querySelectorAll("h2, h3");
    if (headings.length === 0) return;

    const tocList = document.createElement("ul");
    tocList.className = "toc-list";

    headings.forEach((heading, index) => {
      if (!heading.id) {
        heading.id = `heading-${index}`;
      }

      const tocItem = document.createElement("li");
      tocItem.className = `toc-item toc-${heading.tagName.toLowerCase()}`;

      const tocLink = document.createElement("a");
      tocLink.href = `#${heading.id}`;
      tocLink.textContent = heading.textContent;
      tocLink.className = "toc-link";

      tocItem.appendChild(tocLink);
      tocList.appendChild(tocItem);
    });

    tocContainer.appendChild(tocList);
  }

  generateTableOfContents();

  console.log("Cyl Documentation interactions initialized");
});
