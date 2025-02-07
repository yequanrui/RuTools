import locale from '@opentiny/vue-locale';
import { createI18n } from 'vue-i18n';
import enUS from './en-us.json';
import zhCN from './zh-cn.json';

export default (i18n) =>
    locale.initI18n({
        createI18n,
        i18n,
        messages: { zhCN, enUS },
    });
