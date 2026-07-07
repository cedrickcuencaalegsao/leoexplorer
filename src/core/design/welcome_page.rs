pub fn welcome_page_style() -> String {
    format! {"
        .welcome-page {{
            padding: 3rem 2.5rem;
            background: #f5f5f3;
            min-height: 100vh;
        }}
        .welcome-hero {{
            margin-bottom: 2.5rem;
        }}
        .welcome-title {{
            font-size: 28px;
            font-weight: 500;
            color: #2C2C2A;
            margin: 0 0 0.4rem;
        }}
        .welcome-subtitle {{
            font-size: 14px;
            color: #888780;
            margin: 0;
        }}
        .welcome-section-lable {{
            font-size: 11px;
            font-weight: 500;
            letter-spacing: 0.08em;
            text-transform: uppercase;
            color: #B4B2A9;
            margin: 0 0 0.75rem;
        }}
        .welcome-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
            gap: 8px;
            margin-bottom: 2rem;
        }}
        .welcome-card {{
            background: #ffffff;
            border: 0.5px solid #E8E6DF;
            border-radius: 14px;
            padding: 1rem;
            cursor: pointer;
            display: flex;
            flex-direction: column;
            gap: 10px;
            transition: border-color 0.15s, transform 0.1s;
        }}
        .welcome-card:hover {{
            border-color: #B4B2A9;
            transform: translateY(-1px);
        }}
        .welcome-card:active {{
            transform: translateY(0);
        }}
        .welcome-card-icon {{
            width: 36px;
            height: 36px;
            border-radius: 10px;
            background: #F1EFE8;
            display: flex;
            align-items: center;
            justify-content: center;
        }}
        .welcome-card-icon img {{
            width: 20px;
            height: 20px;
            object-fit: contain;
        }}
        .welcome-card-label {{
            font-size: 13px;
            font-weight: 500;
            color: #2C2C2A;
        }}
    "}
}
