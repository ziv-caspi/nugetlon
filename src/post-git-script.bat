cd %FOUND_DIR% 
git branch --force %BRANCH_NAME% 
git checkout %BRANCH_NAME% 
git add . 
git commit -m "%COMMIT_MSG%" 
:: git push