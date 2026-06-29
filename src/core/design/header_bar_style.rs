pub fn header_bar_style() -> String {
    format! {"
        .header-bar {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 4px 10px;
            background-color: #ffffff;
            height: 44px;
            gap: 8px;
            box-shadow: 0 1px 3px rgba(0, 0, 0, 0.06);
        }}
        .nav-button-container {{
            flex: 0 0 auto;
            display: flex;
            align-items: center;
        }}
        .nav-buttons {{
            display: flex;
            flex-direction: row;
            align-items: center;
            gap: 2px;
        }}
        .nav-button {{
            display: flex;
            align-items: center;
            justify-content: center;
            width: 30px;
            height: 30px;
            border-radius: 8px;
            color: rgba(0, 0, 0, 0.45);
            cursor: pointer;
            transition: background-color 0.15s ease, color 0.15s ease;
        }}
        .nav-button:hover {{
            background-color: rgba(0, 0, 0, 0.06);
            color: rgba(0, 0, 0, 0.8);
        }}
        .nav-button:active {{
            background-color: rgba(0, 0, 0, 0.1);
        }}
        .search-bar-container {{
            flex: 1 1 auto;
            display: flex;
            justify-content: center;
            align-items: center;
            min-width: 0;
            width: 100%;
        }}
        .search-bar {{
            display: flex;
            align-items: center;
            gap: 6px;
            width: 100%;
            background-color: #ffffff;
            border: 1px solid rgba(0, 0, 0, 0.12);
            border-radius: 20px;
            padding: 5px 7px;
            transition: border-color 0.15s ease, box-shadow 0.15s ease;
        }}
        .search-bar:focus-within {{
            border-color: rgba(99, 102, 241, 0.5);
            box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.1);
        }}
        .search-bar-icon-wrapper {{
            font-size: 12px;
            display: flex;
            align-items: center;
            color: rgba(0, 0, 0, 0.3);
            flex-shrink: 0;
        }}
        .search-bar-input-wrapper {{
            flex: 1;
            min-width: 0;
        }}
        .search-bar-input-wrapper input {{
            width: 100%;
            background: transparent;
            border: none;
            outline: none;
            color: rgba(0, 0, 0, 0.85);
            font-size: 13.5px;
            font-family: inherit;
            letter-spacing: 0.01em;
        }}
        .search-bar-input-wrapper input::placeholder {{
            color: rgba(0, 0, 0, 0.28);
        }}
        .header-button-container {{
            flex: 0 0 auto;
            display: flex;
            align-items: center;
        }}
        .header-buttons {{
            display: flex;
            flex-direction: row;
            align-items: center;
            gap: 2px;
        }}
        .button {{
            display: flex;
            align-items: center;
            justify-content: center;
            width: 32px;
            height: 32px;
            border-radius: 8px;
            color: rgba(0, 0, 0, 0.45);
            cursor: pointer;
            transition: background-color 0.15s ease, color 0.15s ease;
        }}
        .button:hover {{
            background-color: rgba(0, 0, 0, 0.06);
            color: rgba(0, 0, 0, 0.8);
        }}
        .button:active {{
            background-color: rgba(0, 0, 0, 0.1);
        }}
        .button:last-child {{
            color: #6366f1;
        }}
        .button:last-child:hover {{
            background-color: rgba(99, 102, 241, 0.1);
            color: #4f46e5;
        }}
    "}
}
