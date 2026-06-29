pub fn options_style() -> String {
    format! {"
        .ctx-overlay {{
            position: fixed;
            inset: 0;
            z-index: 999;
        }}
        .folder-ctx-menu {{
            position: fixed;
            z-index: 1000;
            min-width: 220px;
            padding: 4px;
            background: rgba(32, 32, 32, 0.97);
            backdrop-filter: blur(16px);
            border: 1px solid rgba(255, 255, 255, 0.08);
            border-radius: 8px;
            box-shadow:
                0 8px 32px rgba(0, 0, 0, 0.55),
                0 2px 8px rgba(0, 0, 0, 0.3);
            color: #e0e0e0;
            font-size: 13px;
            animation: ctx-pop 0.1s ease;
        }}
        .folder-ctx-menu::-webkit-scrollbar {{
            width: 4px;
        }}
        .folder-ctx-menu::-webkit-scrollbar-track {{
            background: transparent;
        }}
        .folder-ctx-menu::-webkit-scrollbar-thumb {{
            background: rgba(255, 255, 255, 0.15);
            border-radius: 999px;
        }}
        .folder-ctx-menu::-webkit-scrollbar-thumb:hover {{
            background: rgba(255, 255, 255, 0.28);
        }}
        .folder-ctx-menu {{
            scrollbar-width: thin;
            scrollbar-color: rgba(255, 255, 255, 0.15) transparent;
        }}
        @keyframes ctx-pop {{
            from {{ opacity: 0; transform: scale(0.96) translateY(-4px); }}
            to   {{ opacity: 1; transform: scale(1) translateY(0); }}
        }}
        .ctx-section {{
            padding: 2px 0;
        }}
        .ctx-item {{
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 6px 12px;
            border-radius: 4px;
            cursor: pointer;
            user-select: none;
            transition: background 0.08s ease;
        }}
        .ctx-item:hover {{
            background: rgba(255, 255, 255, 0.1);
        }}
        .ctx-item:active {{
            background: rgba(255, 255, 255, 0.06);
        }}
        .ctx-item-icon {{
            width: 16px;
            height: 16px;
            flex-shrink: 0;
        }}
        .ctx-item-label {{
            flex: 1;
        }}
        .ctx-item-shortcut {{
            color: rgba(255, 255, 255, 0.35);
            font-size: 11px;
        }}
        .ctx-divider {{
            border: none;
            border-top: 1px solid rgba(255, 255, 255, 0.08);
            margin: 4px 8px;
        }}
    "}
}
