{
  flake.aspects.devshells.gitHooks = {
    git.hooks = {
      enable = true;
      # runs a commitizen check on the commit message to maintain consistency
      commit-msg.text = "cz check --commit-msg-file $1";
      # uses just to run any project specific lints and builds
      pre-commit.text = "just lint && just build";
      # runs a rebase to origin before a push. Aborts push if a rebase occured
      # the aim is to keep any pushes up to date with origin/main
      pre-push.text = ''
        if [ "$(git rebase origin/main | grep "up to date")" = "" ]; then
          exit 1;
        else
          exit 0;
        fi
      '';
    };
  };
}
