pub fn item_style() -> String {
    format! {"
        .item{{
            border-radius: 8px;
            height: 37px;
            display: flex;
            flex-direction: row;
            align-items: center;
            background-color: #ffffff;
            padding: 0 8px;
            gap: 8px;
        }}
        .item-icon{{
            padding: 4px;
            flex-shrink: 0;
            display: flex;
            align-items: center;
            justify-content: center;
        }}
        .item-data{{
            flex: 1;
            padding: 4px;
            display: flex;
            flex-direction: row;
            align-items: center;
            gap: 8px;
            min-width: 0;
        }}
        .item-name-container{{
            flex: 1;
            min-width: 0;
            overflow: hidden;
            display: flex;
            align-items: center;
        }}
        .item-name{{
            margin: 0;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            font-size: 14px;
            font-weight: 700;
        }}
        .item-flag{{
            display: flex;
            align-items: center;
            justify-content: center;
            flex-shrink: 0;
            width: 76px;
            height: 22px;
            border-radius: 999px;
        }}
        .item-flag.has-flag{{
            background-color: #eef1f5;
            border: 1px solid #e0e4ea;
        }}
        .item-flag-text{{
            margin: 0;
            font-size: 10px;
            font-weight: 600;
            line-height: 1.4;
            color: #5b6472;
            text-transform: uppercase;
            letter-spacing: 0.02em;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }}
        .item-metadata-container{{
            flex-shrink: 0;
            width: 150px;
            display: flex;
            flex-direction: column;
            align-items: flex-start;
            justify-content: center;
            gap: 2px;
        }}
        .item-metadata{{
            margin: 0;
            font-size: 11px;
            color: #666;
            white-space: nowrap;
        }}
        .path{{
            flex-shrink: 0;
            width: 180px;
            min-width: 0;
            overflow: hidden;
            display: flex;
            align-items: center;
        }}
        .item-path{{
            margin: 0;
            font-size: 11px;
            color: #9aa1ab;
            font-style: italic;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }}
    "}
}
