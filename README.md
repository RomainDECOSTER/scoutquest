# ScoutQuest Documentation

Welcome to the comprehensive ScoutQuest documentation website!

## 🚀 What's Included

This documentation website provides:

- **Landing Page** - Beautiful, modern homepage showcasing ScoutQuest features
- **Installation Guide** - Multiple installation methods (Docker, Binary, Cargo, Source)
- **SDK Documentation** - Complete JavaScript/TypeScript and Rust SDK guides
- **API Reference** - REST API documentation
- **Examples** - Real-world usage examples
- **GitHub Pages Deployment** - Automated deployment workflow

## 📁 Structure

```
docs/
├── index.html              # Main landing page
├── docs/                   # Documentation pages
│   ├── installation.html   # Installation guide
│   ├── js-sdk.html        # JavaScript/TypeScript SDK docs
│   ├── rust-sdk.html      # Rust SDK docs (to be created)
│   ├── api-reference.html # REST API docs (to be created)
│   └── ...                # Additional documentation pages
├── assets/
│   ├── css/
│   │   ├── style.css      # Main styles
│   │   └── docs.css       # Documentation-specific styles
│   ├── js/
│   │   ├── main.js        # Main JavaScript functionality
│   │   └── docs.js        # Documentation-specific features
│   └── images/
│       └── logo.svg       # ScoutQuest logo with animation
└── .github/workflows/
    └── docs.yml           # GitHub Pages deployment workflow
```

## 🎨 Features

### Landing Page
- Modern, responsive design
- Interactive navigation
- Feature showcase with icons and descriptions
- Code examples with syntax highlighting
- Getting started guide with tabbed installation
- Examples section with project links

### Documentation Pages
- Sidebar navigation with active page highlighting
- Breadcrumb navigation
- Tabbed code examples for different package managers
- Copy-to-clipboard functionality for code blocks
- Responsive design for all screen sizes
- Search functionality (extendable)

### Interactive Elements
- Mobile-responsive navigation with hamburger menu
- Smooth scrolling between sections
- Animated logo with radar sweep
- Tab switching with persistence
- Copy code buttons with success/error feedback
- Keyboard navigation support

## 🚀 Deployment

The documentation is configured for automatic deployment to GitHub Pages:

1. **Automatic Deployment** - Deploys on pushes to `main` branch when `docs/` folder changes
2. **Manual Deployment** - Can be triggered manually via GitHub Actions
3. **Custom Domain Support** - Ready for custom domain configuration

### Setup GitHub Pages

1. Go to your repository Settings → Pages
2. Source: GitHub Actions
3. The workflow will automatically deploy your documentation

### Custom Domain (Optional)

1. Add a `CNAME` file to the `docs/` folder with your domain
2. Configure DNS for your domain to point to GitHub Pages

## 🛠 Development

### Local Development

To work on the documentation locally:

```bash
# Start a local server in the docs folder
cd docs
python -m http.server 8000
# or
npx serve .

# Visit http://localhost:8000
```

### Adding New Pages

1. Create new HTML file in `docs/docs/` folder
2. Follow the existing template structure
3. Add navigation link to sidebar
4. Update breadcrumb navigation

### Customization

- **Colors** - Update CSS custom properties in `assets/css/style.css`
- **Logo** - Replace `assets/images/logo.svg`
- **Content** - Edit HTML files directly
- **Styling** - Modify CSS files for layout changes

## 📚 Content Guidelines

### Writing Documentation
- Use clear, concise language
- Include code examples for all concepts
- Provide multiple installation/usage methods
- Add troubleshooting sections
- Include real-world examples

### Code Examples
- Always include syntax highlighting
- Provide copy buttons for easy use
- Show both basic and advanced usage
- Include error handling examples
- Use realistic variable names and scenarios

### Responsive Design
- Test on mobile devices
- Ensure navigation works on all screen sizes
- Optimize images and assets
- Use appropriate font sizes and spacing

## 🎯 Next Steps

To complete the documentation:

1. **Add Missing Pages**:
   - Rust SDK documentation (`rust-sdk.html`)
   - REST API reference (`api-reference.html`)
   - Configuration guide (`configuration.html`)
   - Troubleshooting guide (`troubleshooting.html`)
   - Advanced topics (health checking, monitoring, production deployment)

2. **Enhance Features**:
   - Add search functionality
   - Include interactive examples
   - Add video tutorials
   - Create downloadable guides

3. **Content Expansion**:
   - More detailed examples
   - Performance benchmarks
   - Comparison with alternatives
   - Migration guides

## 🔧 Technical Details

### CSS Architecture
- CSS custom properties for theming
- Mobile-first responsive design
- Flexbox and Grid layouts
- Smooth animations and transitions

### JavaScript Features
- Vanilla JavaScript (no frameworks)
- Progressive enhancement
- Accessibility features
- Performance optimizations

### SEO Optimization
- Semantic HTML structure
- Meta tags for social sharing
- Proper heading hierarchy
- Fast loading times

This documentation website provides a solid foundation for ScoutQuest documentation that can grow with your project. The modern design, comprehensive features, and automated deployment make it easy to maintain and expand as needed.
