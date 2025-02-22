module.exports = {
    module: {
        rules: [
            {
                test: /\.rs$/,
                exclude: /node_modules/,
                use: [
                    {
                        loader: "rust-wasmpack-loader",
                    },
                ],
            },
        ]
    }
}