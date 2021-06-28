// https://developers.google.com/web/fundamentals/web-components/best-practices?hl=ja
class AbstractComponent extends HTMLElement {
    constructor() {
        super();

        if (this.constructor == AbstractComponent) {
            throw new Error(`Abstract classes can't be initialized.`);
        }
    }

    connectedCallback() {
        let shadow = this.attachShadow({mode: 'open'});
        let [template, css] = ComponentFactory.getComponent(this.tagName.toLocaleLowerCase());
        shadow.appendChild(css.cloneNode(true));
        shadow.appendChild(template.content.cloneNode(true));
    }

    attributeChangedCallback(attr, oldValue, newValue) {
        if (oldValue !== newValue) {
            this[attr] = newValue;
        }
    }

    render() {}
}

const ComponentEvent = (() => {
    const subscribers = {};

    const publish = (topic, data) => {
        if (!Array.isArray(subscribers[topic])) {
            return;
        }

        subscribers[topic].forEach((callback) => {
            callback(data);
        });
    };

    const subscribe = (topic, callback) => {
        if (!Array.isArray(subscribers[topic])) {
            subscribers[topic] = [];
        }

        subscribers[topic].push(callback);
        const index = subscribers[topic].length - 1;

        return  {
            unsubscribe() {
                subscribers[topic].splice(index, 1);
            }
        }
    };

    return {
        publish, subscribe
    };
})();

const ComponentFactory = (() => {
    const components = {};

    const loadFiles = async (component) => {
        let url = path(component);
        let html = await getTextFromDocument(`${url}.html`);
        let doc = new DOMParser().parseFromString(html, 'text/html');
        let template = doc.getElementById(component);

        if (template === undefined || template == null) {
            throw `Component's template not found.`;
        }

        let css = await getTextFromDocument(`${url}.css`);
        let nodeCss = document.createElement('style');
        nodeCss.innerHTML = css;

        return [template, nodeCss];
    };
    
    const getTextFromDocument = async (url) => {
        let doc = await fetch(url, { importance : "low"});
        let text = await doc.text();
        return text;
    };

    const formatTag = (component) => {
        return component.match(/[A-Z][a-z0-9]+|[0-9][a-z0-9]+/g).join('-').toLocaleLowerCase();
    }
    
    const path = (component) => {
        let value = "";
        if (window.location.href.includes(component)) {
            value = component;
        } else {
            value = `/components/${component}/${component}`;
        }
        return value;
    };

    return {
        getComponent: (componentTag) => {
            return components[componentTag];
        },
        register : async (...componentClasses) => {
            for (let index = 0; index < componentClasses.length; index++) {
                const componentClass = componentClasses[index];

                let tag = formatTag(componentClass.name);
                if (components.hasOwnProperty(tag)) {
                    throw 'Component already registered.';
                }
                
                components[tag] = await loadFiles(tag);
                customElements.define(tag, componentClass);
            }
        }
    };
})();
