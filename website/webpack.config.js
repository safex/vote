var path = require('path');
var webpack = require('webpack');
var CommonsChunkPlugin = webpack.optimize.CommonsChunkPlugin;

module.exports = {
	devtool: 'source-map',
	debug: 'true',

	entry: {
		'angular2' : [
			'rxjs',
			'reflect-metadata',
			'angular2/core',
			'angular2/http',
			'angular2/router'
		],
		'app' : './app/app',
		'submitprop' : './app/submitprop',
		'viewprop' : ['./app/viewprop'],
		'voteprop' : ['./app/voteprop']
	},

	output: {
		path: __dirname + '/build/',
		publicPath: 'build/',
		filename: '[name].js',
		sourceMapFilename: '[name].js.map',
		chunkFilename: '[id].chunk.js'
	},

	resolve: {
		extensions: ['', '.ts', '.js', '.json', '.css', '.html']
	},

	module : {
		loaders: [
			{
				test: /\.ts$/,
				loader: 'ts',
				exclude: [ /node_modules/ ]
			}
		]
	},

	plugins: [
		new CommonsChunkPlugin({name: 'angular2', filename: 'angular2.js', minChunks: Infinity}),
		new CommonsChunkPlugin({name: 'common', filename: 'common.js'})
	]
}