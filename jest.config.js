module.exports = {
    transform: {
      '^.+\\.svelte$': 'svelte-jester',
      '^.+\\.js$': 'babel-jest',
    },
    moduleFileExtensions: ['js', 'svelte'],
    testEnvironment: "jsdom",
    transformIgnorePatterns: [
      "node_modules/(?!@ngrx|(?!deck.gl)|ng-dynamic)"
    ]
}