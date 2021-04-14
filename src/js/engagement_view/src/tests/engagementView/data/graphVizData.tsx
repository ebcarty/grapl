export const vizGraphData = [
    {
        uid: 40,
        node_key: "53e2abc7-ce21-4be0-b70a-4c835eefecd1",
        dgraph_type: ["Asset"],
        display: "Asset",
        hostname: "DESKTOP-FVSHABR",
        asset_ip: null,
        asset_processes: [
            {
                uid: 171,
                node_key: "9388ceda-91fc-49c8-989c-8968ad93d0e8",
                dgraph_type: ["Process"],
                display: null,
                process_name: "dropper.exe",
                process_id: 4164,
            },
            {
                uid: 193,
                node_key: "a9ca278c-0f52-4e89-821c-876d750ad1dd",
                dgraph_type: ["Process"],
                display: null,
                process_name: "cmd.exe",
                process_id: 5824,
            },
            {
                uid: 201,
                node_key: "a0c5dacc-fd53-450f-bb99-207467b9c7ee",
                dgraph_type: ["Process"],
                display: null,
                process_name: "cmd.exe",
                process_id: 5824,
            },
            {
                uid: 207,
                node_key: "739c609a-05ac-4bed-949c-387efb06114a",
                dgraph_type: ["Process"],
                display: null,
                process_name: "svchost.exe",
                process_id: 6132,
            },
        ],
        files_on_asset: null,
        risks: [
            {
                uid: 213,
                dgraph_type: ["Risk"],
                display: null,
                node_key: "Rare Parent of cmd.exe",
                analyzer_name: "Rare Parent of cmd.exe",
                risk_score: 10,
            },
            {
                uid: 220,
                dgraph_type: ["Risk"],
                display: null,
                node_key: "Suspicious svchost",
                analyzer_name: "Suspicious svchost",
                risk_score: 75,
            },
        ],
    },
    {
        uid: 171,
        node_key: "9388ceda-91fc-49c8-989c-8968ad93d0e8",
        dgraph_type: ["Process"],
        process_name: "dropper.exe",
        process_id: 4164,
        display: "Process",
        children: [
            {
                uid: 201,
                node_key: "a0c5dacc-fd53-450f-bb99-207467b9c7ee",
                dgraph_type: ["Process"],
                display: null,
                process_name: "cmd.exe",
                process_id: 5824,
            },
        ],
        risks: [
            {
                uid: 213,
                dgraph_type: ["Risk"],
                node_key: "Rare Parent of cmd.exe",
                analyzer_name: "Rare Parent of cmd.exe",
                risk_score: 10,
            },
        ],
    },
    {
        uid: 193,
        node_key: "a9ca278c-0f52-4e89-821c-876d750ad1dd",
        dgraph_type: ["Process"],
        process_name: "cmd.exe",
        process_id: 5824,
        display: "Process",
        children: [
            {
                uid: 207,
                node_key: "739c609a-05ac-4bed-949c-387efb06114a",
                dgraph_type: ["Process"],
                display: null,
                process_name: "svchost.exe",
                process_id: 6132,
            },
        ],
        risks: [
            {
                uid: 220,
                dgraph_type: ["Risk"],
                node_key: "Suspicious svchost",
                analyzer_name: "Suspicious svchost",
                risk_score: 75,
            },
        ],
    },
    {
        uid: 201,
        node_key: "a0c5dacc-fd53-450f-bb99-207467b9c7ee",
        dgraph_type: ["Process"],
        process_name: "cmd.exe",
        process_id: 5824,
        display: "Process",
        children: null,
        risks: [
            {
                uid: 213,
                dgraph_type: ["Risk"],
                node_key: "Rare Parent of cmd.exe",
                analyzer_name: "Rare Parent of cmd.exe",
                risk_score: 10,
            },
        ],
    },
    {
        uid: 207,
        node_key: "739c609a-05ac-4bed-949c-387efb06114a",
        dgraph_type: ["Process"],
        process_name: "svchost.exe",
        process_id: 6132,
        display: "Process",
        children: null,
        risks: null,
    },
];

export const vizGraphReturnData = {
    nodes: [
        {
            name: 40,
            uid: 40,
            node_key: "53e2abc7-ce21-4be0-b70a-4c835eefecd1",
            dgraph_type: ["Asset"],
            display: "Asset",
            hostname: "DESKTOP-FVSHABR",
            asset_ip: null,
            files_on_asset: null,
            risk_score: 85,
            analyzerNames: "Rare Parent of cmd.exe, Suspicious svchost",
            id: 40,
            nodeType: "Asset",
            nodeLabel: "Asset",
        },
        {
            name: 171,
            uid: 171,
            node_key: "9388ceda-91fc-49c8-989c-8968ad93d0e8",
            dgraph_type: ["Process"],
            process_name: "dropper.exe",
            process_id: 4164,
            display: "Process",
            risk_score: 10,
            analyzerNames: "Rare Parent of cmd.exe",
            id: 171,
            nodeType: "Process",
            nodeLabel: "Process",
        },
        {
            name: 193,
            uid: 193,
            node_key: "a9ca278c-0f52-4e89-821c-876d750ad1dd",
            dgraph_type: ["Process"],
            process_name: "cmd.exe",
            process_id: 5824,
            display: "Process",
            risk_score: 75,
            analyzerNames: "Suspicious svchost",
            id: 193,
            nodeType: "Process",
            nodeLabel: "Process",
        },
        {
            name: 201,
            uid: 201,
            node_key: "a0c5dacc-fd53-450f-bb99-207467b9c7ee",
            dgraph_type: ["Process"],
            process_name: "cmd.exe",
            process_id: 5824,
            display: "Process",
            children: null,
            risk_score: 10,
            analyzerNames: "Rare Parent of cmd.exe",
            id: 201,
            nodeType: "Process",
            nodeLabel: "Process",
        },
        {
            name: 207,
            uid: 207,
            node_key: "739c609a-05ac-4bed-949c-387efb06114a",
            dgraph_type: ["Process"],
            process_name: "svchost.exe",
            process_id: 6132,
            display: "Process",
            children: null,
            risks: null,
            risk_score: 0,
            analyzerNames: "",
            id: 207,
            nodeType: "Process",
            nodeLabel: "Process",
        },
    ],
    links: [
        { source: 40, name: "asset_processes", target: 171 },
        { source: 40, name: "asset_processes", target: 193 },
        { source: 40, name: "asset_processes", target: 201 },
        { source: 40, name: "asset_processes", target: 207 },
        { source: 171, name: "children", target: 201 },
        { source: 193, name: "children", target: 207 },
    ],
    index: {
        "40": {
            name: 40,
            uid: 40,
            node_key: "53e2abc7-ce21-4be0-b70a-4c835eefecd1",
            dgraph_type: ["Asset"],
            display: "Asset",
            hostname: "DESKTOP-FVSHABR",
            asset_ip: null,
            files_on_asset: null,
            risk_score: 85,
            analyzerNames: "Rare Parent of cmd.exe, Suspicious svchost",
            id: 40,
            nodeType: "Asset",
            nodeLabel: "Asset",
        },
        "171": {
            name: 171,
            uid: 171,
            node_key: "9388ceda-91fc-49c8-989c-8968ad93d0e8",
            dgraph_type: ["Process"],
            process_name: "dropper.exe",
            process_id: 4164,
            display: "Process",
            risk_score: 10,
            analyzerNames: "Rare Parent of cmd.exe",
            id: 171,
            nodeType: "Process",
            nodeLabel: "Process",
        },
        "193": {
            name: 193,
            uid: 193,
            node_key: "a9ca278c-0f52-4e89-821c-876d750ad1dd",
            dgraph_type: ["Process"],
            process_name: "cmd.exe",
            process_id: 5824,
            display: "Process",
            risk_score: 75,
            analyzerNames: "Suspicious svchost",
            id: 193,
            nodeType: "Process",
            nodeLabel: "Process",
        },
        "201": {
            name: 201,
            uid: 201,
            node_key: "a0c5dacc-fd53-450f-bb99-207467b9c7ee",
            dgraph_type: ["Process"],
            process_name: "cmd.exe",
            process_id: 5824,
            display: "Process",
            children: null,
            risk_score: 10,
            analyzerNames: "Rare Parent of cmd.exe",
            id: 201,
            nodeType: "Process",
            nodeLabel: "Process",
        },
        "207": {
            name: 207,
            uid: 207,
            node_key: "739c609a-05ac-4bed-949c-387efb06114a",
            dgraph_type: ["Process"],
            process_name: "svchost.exe",
            process_id: 6132,
            display: "Process",
            children: null,
            risks: null,
            risk_score: 0,
            analyzerNames: "",
            id: 207,
            nodeType: "Process",
            nodeLabel: "Process",
        },
    },
};
