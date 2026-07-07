pub fn access_denied_style() -> String {
    format! {"
        .access-denied {{
            display: flex;
            align-items: center;
            justify-content: center;
            height: 100%;
            width: 100%;
            background: rgba(0, 0, 0, 0.85);
        }}
        .access-denied-box {{
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 12px;
            padding: 40px;
            background: rgba(32, 32, 32, 0.97);
            border: 1px solid rgba(255, 80, 80, 0.3);
            border-radius: 12px;
            color: #e0e0e0;
            text-align: center;
        }}
        .access-denied-icon {{
            font-size: 48px;
        }}
        .access-denied-box h2 {{
            color: #ff5f5f;
            margin: 0;
        }}
        .access-denied-box p {{
            color: rgba(255, 255, 255, 0.5);
            margin: 0;
            font-size: 13px;
        }}
        .access-denied-role {{
            font-size: 11px !important;
            color: rgba(255, 255, 255, 0.25) !important;
        }}
    "}
}
