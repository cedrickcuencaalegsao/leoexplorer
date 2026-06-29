pub fn welcome_page_style() -> String {
    format! {
        "
        .welcome-page {{
            padding: 2rem 1.5rem;
        }}
        .welcome-app-name {{
            font-size: 12px;
            font-weight: 500;
            text-transform: uppercase;
            letter-spacing: 0.08em;
            color: var(--muted);
            margin: 0 0 0.25rem;
        }}
        .welcome-title {{
            font-size: 22px;
            font-weight: 500;
            margin: 0 0 0.5rem;
        }}
        .welcome-subtitle {{
            font-size: 15px;
            color: var(--muted);
            margin: 0 0 2rem;
        }}
        .welcome-section-label {{
            font-size: 12px;
            font-weight: 500;
            text-transform: uppercase;
            letter-spacing: 0.08em;
            color: var(--muted);
            margin: 0 0 0.75rem;
        }}
        .welcome-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
            gap: 10px;
            margin-bottom: 1.5rem;
        }}
        .welcome-card {{
            background: var(--bg-surface);
            border: 0.5px solid var(--border);
            border-radius: 12px;
            padding: 1rem;
            cursor: pointer;
            display: flex;
            flex-direction: column;
            gap: 6px;
            transition: background 0.15s;
        }}
        .welcome-card:hover {{ background: var(--bg-hover); }}
        .welcome-card-title {{ font-size: 14px; font-weight: 500; }}
        .welcome-card-desc  {{ font-size: 12px; color: var(--muted); }}
        "
    }
}
