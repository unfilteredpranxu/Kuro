import {test,expect} from '@playwright/test';
for(const [name,width,height] of [['desktop',1440,900],['ipad-landscape',1180,820],['ipad-portrait',820,1180],['iphone',390,844],['narrow',320,700]]){
 test(`layout ${name}`,async({page})=>{
  await page.setViewportSize({width,height});await page.goto('/');await expect(page.getByRole('heading',{name:'Make room for understanding.'})).toBeVisible();await page.evaluate(()=>document.fonts.ready);
  expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
  await expect(page).toHaveScreenshot(`${name}.png`,{fullPage:true,animations:'disabled',mask:[page.locator('.date-stamp strong')],maxDiffPixelRatio:0.002});
 });
}
