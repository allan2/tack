# tack

A minimal syntax highlighter that outputs HTML.

**Note: this library is still alpha and subject to breaking API changes.** 

Example:
```css
body {
	display: flex;
	flex-direction: column;
	min-height: 100%;
}
```

Output:
```html
<span class="tag">body</span> {
        <span class="property">display</span>: <span class="string">flex</span>;
        <span class="property">flex-direction</span>: <span class="string">column</span>;
        <span class="property">min-height</span>: <span class="numeric">100%</span>;
}
```

Supported languages:
 - CSS
