use router::trie::Trie;

#[path = "../fixtures/github.rs"]
mod github;

#[allow(clippy::too_many_lines)]
#[test]
fn test() {
    let mut trie = Trie::new();
    let mut counter = 0;
    for route in github::ROUTES {
        counter += 1;
        trie.insert(route, counter);
    }

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ / [1]
          ├─ a
          │  ├─ dvisories [2]
          │  │          ╰─ /
          │  │             ╰─ {ghsa_id} [3]
          │  ├─ pp [4]
          │  │   ├─ -manifests/
          │  │   │            ╰─ {code}
          │  │   │                    ╰─ /conversions [5]
          │  │   ├─ /
          │  │   │  ├─ hook/
          │  │   │  │      ├─ config [6]
          │  │   │  │      ╰─ deliveries [7]
          │  │   │  │                  ╰─ /
          │  │   │  │                     ╰─ {delivery_id} [8]
          │  │   │  │                                    ╰─ /attempts [9]
          │  │   │  ╰─ installation
          │  │   │                ├─ -requests [10]
          │  │   │                ╰─ s [11]
          │  │   │                   ╰─ /
          │  │   │                      ╰─ {installation_id} [12]
          │  │   │                                         ╰─ /
          │  │   │                                            ├─ access_tokens [13]
          │  │   │                                            ╰─ suspended [14]
          │  │   ├─ lications/
          │  │   │           ╰─ {client_id}
          │  │   │                        ╰─ /
          │  │   │                           ├─ grant [15]
          │  │   │                           ╰─ token [16]
          │  │   │                                  ╰─ /scoped [17]
          │  │   ╰─ s/
          │  │       ╰─ {app_slug} [18]
          │  ╰─ ssignments/
          │               ╰─ {assignment_id} [19]
          │                                ╰─ /
          │                                   ├─ accepted_assignments [20]
          │                                   ╰─ grades [21]
          ├─ c
          │  ├─ lassrooms [22]
          │  │          ╰─ /
          │  │             ╰─ {classroom_id} [23]
          │  │                             ╰─ /assignments [24]
          │  ╰─ odes_of_conduct [25]
          │                   ╰─ /
          │                      ╰─ {key} [26]
          ├─ e
          │  ├─ mojis [27]
          │  ├─ nterprises/
          │  │            ╰─ {enterprise}
          │  │                          ╰─ /
          │  │                             ├─ copilot/usage [28]
          │  │                             ├─ dependabot/alerts [29]
          │  │                             ╰─ secret-scanning/alerts [30]
          │  ╰─ vents [31]
          ├─ feeds [32]
          ├─ gi
          │   ├─ sts [33]
          │   │    ╰─ /
          │   │       ├─ public [34]
          │   │       ├─ starred [35]
          │   │       ╰─ {gist_id} [36]
          │   │                  ╰─ /
          │   │                     ├─ comm
          │   │                     │     ├─ ents [37]
          │   │                     │     │     ╰─ /
          │   │                     │     │        ╰─ {comment_id} [38]
          │   │                     │     ╰─ its [39]
          │   │                     ├─ forks [40]
          │   │                     ├─ star [41]
          │   │                     ╰─ {sha} [42]
          │   ╰─ tignore/templates [43]
          │                      ╰─ /
          │                         ╰─ {name} [44]
          ├─ i
          │  ├─ nstallation/
          │  │             ├─ repositories [45]
          │  │             ╰─ token [46]
          │  ╰─ ssues [47]
          ├─ licenses [48]
          │         ╰─ /
          │            ╰─ {license} [49]
          ├─ m
          │  ├─ ark
          │  │    ├─ down [50]
          │  │    │     ╰─ /raw [51]
          │  │    ╰─ etplace_listing/
          │  │                      ├─ accounts/
          │  │                      │          ╰─ {account_id} [52]
          │  │                      ├─ plans [53]
          │  │                      │      ╰─ /
          │  │                      │         ╰─ {plan_id}
          │  │                      │                    ╰─ /accounts [54]
          │  │                      ╰─ stubbed/
          │  │                                ├─ accounts/
          │  │                                │          ╰─ {account_id} [55]
          │  │                                ╰─ plans [56]
          │  │                                       ╰─ /
          │  │                                          ╰─ {plan_id}
          │  │                                                     ╰─ /accounts [57]
          │  ╰─ eta [58]
          ├─ n
          │  ├─ etworks/
          │  │         ╰─ {owner}
          │  │                  ╰─ /
          │  │                     ╰─ {repo}
          │  │                             ╰─ /events [59]
          │  ╰─ otifications [60]
          │                ╰─ /threads/
          │                           ╰─ {thread_id} [61]
          │                                        ╰─ /subscription [62]
          ├─ o
          │  ├─ ctocat [63]
          │  ╰─ rg
          │      ├─ anizations [64]
          │      ╰─ s/
          │          ╰─ {org} [65]
          │                 ╰─ /
          │                    ├─ actions/
          │                    │         ├─ cache/usage [66]
          │                    │         │            ╰─ -by-repository [67]
          │                    │         ├─ oidc/customization/sub [68]
          │                    │         ├─ permissions [69]
          │                    │         │            ╰─ /
          │                    │         │               ├─ repositories [70]
          │                    │         │               │             ╰─ /
          │                    │         │               │                ╰─ {repository_id} [71]
          │                    │         │               ├─ selected-actions [72]
          │                    │         │               ╰─ workflow [73]
          │                    │         ├─ runners [74]
          │                    │         │        ╰─ /
          │                    │         │           ├─ downloads [75]
          │                    │         │           ├─ generate-jitconfig [76]
          │                    │         │           ├─ re
          │                    │         │           │   ├─ gistration-token [77]
          │                    │         │           │   ╰─ move-token [78]
          │                    │         │           ╰─ {runner_id} [79]
          │                    │         │                        ╰─ /labels [80]
          │                    │         │                                 ╰─ /
          │                    │         │                                    ╰─ {name} [81]
          │                    │         ├─ secrets [82]
          │                    │         │        ╰─ /
          │                    │         │           ├─ public-key [83]
          │                    │         │           ╰─ {secret_name} [84]
          │                    │         │                          ╰─ /repositories [85]
          │                    │         │                                         ╰─ /
          │                    │         │                                            ╰─ {repository_id} [86]
          │                    │         ╰─ variables [87]
          │                    │                    ╰─ /
          │                    │                       ╰─ {name} [88]
          │                    │                               ╰─ /repositories [89]
          │                    │                                              ╰─ /
          │                    │                                                 ╰─ {repository_id} [90]
          │                    ├─ blocks [91]
          │                    │       ╰─ /
          │                    │          ╰─ {username} [92]
          │                    ├─ co
          │                    │   ├─ de
          │                    │   │   ├─ -scanning/alerts [93]
          │                    │   │   ╰─ spaces [94]
          │                    │   │           ╰─ /
          │                    │   │              ├─ access [95]
          │                    │   │              │       ╰─ /selected_users [96]
          │                    │   │              ╰─ secrets [97]
          │                    │   │                       ╰─ /
          │                    │   │                          ├─ public-key [98]
          │                    │   │                          ╰─ {secret_name} [99]
          │                    │   │                                         ╰─ /repositories [100]
          │                    │   │                                                        ╰─ /
          │                    │   │                                                           ╰─ {repository_id} [101]
          │                    │   ╰─ pilot/
          │                    │           ├─ billing [102]
          │                    │           │        ╰─ /se
          │                    │           │             ├─ ats [103]
          │                    │           │             ╰─ lected_
          │                    │           │                      ├─ teams [104]
          │                    │           │                      ╰─ users [105]
          │                    │           ╰─ usage [106]
          │                    ├─ d
          │                    │  ├─ ependabot/
          │                    │  │           ├─ alerts [107]
          │                    │  │           ╰─ secrets [108]
          │                    │  │                    ╰─ /
          │                    │  │                       ├─ public-key [109]
          │                    │  │                       ╰─ {secret_name} [110]
          │                    │  │                                      ╰─ /repositories [111]
          │                    │  │                                                     ╰─ /
          │                    │  │                                                        ╰─ {repository_id} [112]
          │                    │  ╰─ ocker/conflicts [113]
          │                    ├─ events [114]
          │                    ├─ failed_invitations [115]
          │                    ├─ hooks [116]
          │                    │      ╰─ /
          │                    │         ╰─ {hook_id} [117]
          │                    │                    ╰─ /
          │                    │                       ├─ config [118]
          │                    │                       ├─ deliveries [119]
          │                    │                       │           ╰─ /
          │                    │                       │              ╰─ {delivery_id} [120]
          │                    │                       │                             ╰─ /attempts [121]
          │                    │                       ╰─ pings [122]
          │                    ├─ i
          │                    │  ├─ n
          │                    │  │  ├─ stallation [123]
          │                    │  │  │           ╰─ s [124]
          │                    │  │  ├─ teraction-limits [125]
          │                    │  │  ╰─ vitations [126]
          │                    │  │             ╰─ /
          │                    │  │                ╰─ {invitation_id} [127]
          │                    │  │                                 ╰─ /teams [128]
          │                    │  ╰─ ssues [129]
          │                    ├─ m
          │                    │  ├─ embers [130]
          │                    │  │       ├─ /
          │                    │  │       │  ╰─ {username} [131]
          │                    │  │       │              ╰─ /co
          │                    │  │       │                   ├─ despaces [132]
          │                    │  │       │                   │         ╰─ /
          │                    │  │       │                   │            ╰─ {codespace_name} [133]
          │                    │  │       │                   │                              ╰─ /stop [134]
          │                    │  │       │                   ╰─ pilot [135]
          │                    │  │       ╰─ hips/
          │                    │  │              ╰─ {username} [136]
          │                    │  ╰─ igrations [137]
          │                    │             ╰─ /
          │                    │                ╰─ {migration_id} [138]
          │                    │                                ╰─ /
          │                    │                                   ├─ archive [139]
          │                    │                                   ╰─ repos
          │                    │                                          ├─ /
          │                    │                                          │  ╰─ {repo_name}
          │                    │                                          │               ╰─ /lock [140]
          │                    │                                          ╰─ itories [141]
          │                    ├─ o
          │                    │  ├─ rganization-
          │                    │  │             ├─ fine-grained-permissions [142]
          │                    │  │             ╰─ roles [143]
          │                    │  │                    ╰─ /
          │                    │  │                       ├─ teams/
          │                    │  │                       │       ╰─ {team_slug} [144]
          │                    │  │                       │                    ╰─ /
          │                    │  │                       │                       ╰─ {role_id} [145]
          │                    │  │                       ├─ users/
          │                    │  │                       │       ╰─ {username} [146]
          │                    │  │                       │                   ╰─ /
          │                    │  │                       │                      ╰─ {role_id} [147]
          │                    │  │                       ╰─ {role_id} [148]
          │                    │  │                                  ╰─ /
          │                    │  │                                     ├─ teams [149]
          │                    │  │                                     ╰─ users [150]
          │                    │  ╰─ utside_collaborators [151]
          │                    │                        ╰─ /
          │                    │                           ╰─ {username} [152]
          │                    ├─ p
          │                    │  ├─ ackages [153]
          │                    │  │        ╰─ /
          │                    │  │           ╰─ {package_type}
          │                    │  │                           ╰─ /
          │                    │  │                              ╰─ {package_name} [154]
          │                    │  │                                              ╰─ /
          │                    │  │                                                 ├─ restore [155]
          │                    │  │                                                 ╰─ versions [156]
          │                    │  │                                                           ╰─ /
          │                    │  │                                                              ╰─ {package_version_id} [157]
          │                    │  │                                                                                    ╰─ /restore [158]
          │                    │  ├─ ersonal-access-token
          │                    │  │                     ├─ -requests [159]
          │                    │  │                     │          ╰─ /
          │                    │  │                     │             ╰─ {pat_request_id} [160]
          │                    │  │                     │                               ╰─ /repositories [161]
          │                    │  │                     ╰─ s [162]
          │                    │  │                        ╰─ /
          │                    │  │                           ╰─ {pat_id} [163]
          │                    │  │                                     ╰─ /repositories [164]
          │                    │  ├─ ro
          │                    │  │   ├─ jects [165]
          │                    │  │   ╰─ perties/
          │                    │  │             ├─ schema [166]
          │                    │  │             │       ╰─ /
          │                    │  │             │          ╰─ {custom_property_name} [167]
          │                    │  │             ╰─ values [168]
          │                    │  ╰─ ublic_members [169]
          │                    │                 ╰─ /
          │                    │                    ╰─ {username} [170]
          │                    ├─ r
          │                    │  ├─ epos [171]
          │                    │  ╰─ ulesets [172]
          │                    │           ╰─ /
          │                    │              ├─ rule-suites [173]
          │                    │              │            ╰─ /
          │                    │              │               ╰─ {rule_suite_id} [174]
          │                    │              ╰─ {ruleset_id} [175]
          │                    ├─ se
          │                    │   ├─ c
          │                    │   │  ├─ ret-scanning/alerts [176]
          │                    │   │  ╰─ urity-
          │                    │   │          ├─ advisories [177]
          │                    │   │          ╰─ managers [178]
          │                    │   │                    ╰─ /teams/
          │                    │   │                             ╰─ {team_slug} [179]
          │                    │   ╰─ ttings/billing/
          │                    │                    ├─ actions [180]
          │                    │                    ├─ packages [181]
          │                    │                    ╰─ shared-storage [182]
          │                    ├─ teams [183]
          │                    │      ╰─ /
          │                    │         ╰─ {team_slug} [184]
          │                    │                      ╰─ /
          │                    │                         ├─ discussions [185]
          │                    │                         │            ╰─ /
          │                    │                         │               ╰─ {discussion_number} [186]
          │                    │                         │                                    ╰─ /
          │                    │                         │                                       ├─ comments [187]
          │                    │                         │                                       │         ╰─ /
          │                    │                         │                                       │            ╰─ {comment_number} [188]
          │                    │                         │                                       │                              ╰─ /reactions [189]
          │                    │                         │                                       │                                          ╰─ /
          │                    │                         │                                       │                                             ╰─ {reaction_id} [190]
          │                    │                         │                                       ╰─ reactions [191]
          │                    │                         │                                                  ╰─ /
          │                    │                         │                                                     ╰─ {reaction_id} [192]
          │                    │                         ├─ invitations [193]
          │                    │                         ├─ members [194]
          │                    │                         │        ╰─ hips/
          │                    │                         │               ╰─ {username} [195]
          │                    │                         ├─ projects [196]
          │                    │                         │         ╰─ /
          │                    │                         │            ╰─ {project_id} [197]
          │                    │                         ├─ repos [198]
          │                    │                         │      ╰─ /
          │                    │                         │         ╰─ {owner}
          │                    │                         │                  ╰─ /
          │                    │                         │                     ╰─ {repo} [199]
          │                    │                         ╰─ teams [200]
          │                    ╰─ {security_product}
          │                                        ╰─ /
          │                                           ╰─ {enablement} [201]
          ├─ projects/
          │          ├─ columns/
          │          │         ├─ cards/
          │          │         │       ╰─ {card_id} [202]
          │          │         │                  ╰─ /moves [203]
          │          │         ╰─ {column_id} [204]
          │          │                      ╰─ /
          │          │                         ├─ cards [205]
          │          │                         ╰─ moves [206]
          │          ╰─ {project_id} [207]
          │                        ╰─ /col
          │                              ├─ laborators [208]
          │                              │           ╰─ /
          │                              │              ╰─ {username} [209]
          │                              │                          ╰─ /permission [210]
          │                              ╰─ umns [211]
          ├─ r
          │  ├─ ate_limit [212]
          │  ╰─ epos
          │        ├─ /
          │        │  ├─ {owner}
          │        │  │        ╰─ /
          │        │  │           ╰─ {repo} [213]
          │        │  │                   ╰─ /
          │        │  │                      ├─ a
          │        │  │                      │  ├─ cti
          │        │  │                      │  │    ├─ ons/
          │        │  │                      │  │    │     ├─ artifacts [214]
          │        │  │                      │  │    │     │          ╰─ /
          │        │  │                      │  │    │     │             ╰─ {artifact_id} [215]
          │        │  │                      │  │    │     │                            ╰─ /
          │        │  │                      │  │    │     │                               ╰─ {archive_format} [216]
          │        │  │                      │  │    │     ├─ cache
          │        │  │                      │  │    │     │      ├─ /usage [217]
          │        │  │                      │  │    │     │      ╰─ s [218]
          │        │  │                      │  │    │     │         ╰─ /
          │        │  │                      │  │    │     │            ╰─ {cache_id} [219]
          │        │  │                      │  │    │     ├─ jobs/
          │        │  │                      │  │    │     │      ╰─ {job_id} [220]
          │        │  │                      │  │    │     │                ╰─ /
          │        │  │                      │  │    │     │                   ├─ logs [221]
          │        │  │                      │  │    │     │                   ╰─ rerun [222]
          │        │  │                      │  │    │     ├─ o
          │        │  │                      │  │    │     │  ├─ idc/customization/sub [223]
          │        │  │                      │  │    │     │  ╰─ rganization-
          │        │  │                      │  │    │     │                ├─ secrets [224]
          │        │  │                      │  │    │     │                ╰─ variables [225]
          │        │  │                      │  │    │     ├─ permissions [226]
          │        │  │                      │  │    │     │            ╰─ /
          │        │  │                      │  │    │     │               ├─ access [227]
          │        │  │                      │  │    │     │               ├─ selected-actions [228]
          │        │  │                      │  │    │     │               ╰─ workflow [229]
          │        │  │                      │  │    │     ├─ run
          │        │  │                      │  │    │     │    ├─ ners [230]
          │        │  │                      │  │    │     │    │     ╰─ /
          │        │  │                      │  │    │     │    │        ├─ downloads [231]
          │        │  │                      │  │    │     │    │        ├─ generate-jitconfig [232]
          │        │  │                      │  │    │     │    │        ├─ re
          │        │  │                      │  │    │     │    │        │   ├─ gistration-token [233]
          │        │  │                      │  │    │     │    │        │   ╰─ move-token [234]
          │        │  │                      │  │    │     │    │        ╰─ {runner_id} [235]
          │        │  │                      │  │    │     │    │                     ╰─ /labels [236]
          │        │  │                      │  │    │     │    │                              ╰─ /
          │        │  │                      │  │    │     │    │                                 ╰─ {name} [237]
          │        │  │                      │  │    │     │    ╰─ s [238]
          │        │  │                      │  │    │     │       ╰─ /
          │        │  │                      │  │    │     │          ╰─ {run_id} [239]
          │        │  │                      │  │    │     │                    ╰─ /
          │        │  │                      │  │    │     │                       ├─ a
          │        │  │                      │  │    │     │                       │  ├─ pprov
          │        │  │                      │  │    │     │                       │  │      ├─ als [240]
          │        │  │                      │  │    │     │                       │  │      ╰─ e [241]
          │        │  │                      │  │    │     │                       │  ├─ rtifacts [242]
          │        │  │                      │  │    │     │                       │  ╰─ ttempts/
          │        │  │                      │  │    │     │                       │            ╰─ {attempt_number} [243]
          │        │  │                      │  │    │     │                       │                              ╰─ /
          │        │  │                      │  │    │     │                       │                                 ├─ jobs [244]
          │        │  │                      │  │    │     │                       │                                 ╰─ logs [245]
          │        │  │                      │  │    │     │                       ├─ cancel [246]
          │        │  │                      │  │    │     │                       ├─ deployment_protection_rule [247]
          │        │  │                      │  │    │     │                       ├─ force-cancel [248]
          │        │  │                      │  │    │     │                       ├─ jobs [249]
          │        │  │                      │  │    │     │                       ├─ logs [250]
          │        │  │                      │  │    │     │                       ├─ pending_deployments [251]
          │        │  │                      │  │    │     │                       ├─ rerun [252]
          │        │  │                      │  │    │     │                       │      ╰─ -failed-jobs [253]
          │        │  │                      │  │    │     │                       ╰─ timing [254]
          │        │  │                      │  │    │     ├─ secrets [255]
          │        │  │                      │  │    │     │        ╰─ /
          │        │  │                      │  │    │     │           ├─ public-key [256]
          │        │  │                      │  │    │     │           ╰─ {secret_name} [257]
          │        │  │                      │  │    │     ├─ variables [258]
          │        │  │                      │  │    │     │          ╰─ /
          │        │  │                      │  │    │     │             ╰─ {name} [259]
          │        │  │                      │  │    │     ╰─ workflows [260]
          │        │  │                      │  │    │                ╰─ /
          │        │  │                      │  │    │                   ╰─ {workflow_id} [261]
          │        │  │                      │  │    │                                  ╰─ /
          │        │  │                      │  │    │                                     ├─ dis
          │        │  │                      │  │    │                                     │    ├─ able [262]
          │        │  │                      │  │    │                                     │    ╰─ patches [263]
          │        │  │                      │  │    │                                     ├─ enable [264]
          │        │  │                      │  │    │                                     ├─ runs [265]
          │        │  │                      │  │    │                                     ╰─ timing [266]
          │        │  │                      │  │    ╰─ vity [267]
          │        │  │                      │  ├─ ssignees [268]
          │        │  │                      │  │         ╰─ /
          │        │  │                      │  │            ╰─ {assignee} [269]
          │        │  │                      │  ╰─ uto
          │        │  │                      │       ├─ links [270]
          │        │  │                      │       │      ╰─ /
          │        │  │                      │       │         ╰─ {autolink_id} [271]
          │        │  │                      │       ╰─ mated-security-fixes [272]
          │        │  │                      ├─ branches [273]
          │        │  │                      │         ╰─ /
          │        │  │                      │            ╰─ {branch} [274]
          │        │  │                      │                      ╰─ /
          │        │  │                      │                         ├─ protection [275]
          │        │  │                      │                         │           ╰─ /
          │        │  │                      │                         │              ├─ enforce_admins [276]
          │        │  │                      │                         │              ╰─ re
          │        │  │                      │                         │                  ├─ quired_
          │        │  │                      │                         │                  │        ├─ pull_request_reviews [277]
          │        │  │                      │                         │                  │        ╰─ s
          │        │  │                      │                         │                  │           ├─ ignatures [278]
          │        │  │                      │                         │                  │           ╰─ tatus_checks [279]
          │        │  │                      │                         │                  │                         ╰─ /contexts [280]
          │        │  │                      │                         │                  ╰─ strictions [281]
          │        │  │                      │                         │                              ╰─ /
          │        │  │                      │                         │                                 ├─ apps [282]
          │        │  │                      │                         │                                 ├─ teams [283]
          │        │  │                      │                         │                                 ╰─ users [284]
          │        │  │                      │                         ╰─ rename [285]
          │        │  │                      ├─ c
          │        │  │                      │  ├─ heck-
          │        │  │                      │  │      ├─ runs [286]
          │        │  │                      │  │      │     ╰─ /
          │        │  │                      │  │      │        ╰─ {check_run_id} [287]
          │        │  │                      │  │      │                        ╰─ /
          │        │  │                      │  │      │                           ├─ annotations [288]
          │        │  │                      │  │      │                           ╰─ rerequest [289]
          │        │  │                      │  │      ╰─ suites [290]
          │        │  │                      │  │              ╰─ /
          │        │  │                      │  │                 ├─ preferences [291]
          │        │  │                      │  │                 ╰─ {check_suite_id} [292]
          │        │  │                      │  │                                   ╰─ /
          │        │  │                      │  │                                      ├─ check-runs [293]
          │        │  │                      │  │                                      ╰─ rerequest [294]
          │        │  │                      │  ╰─ o
          │        │  │                      │     ├─ de
          │        │  │                      │     │   ├─ -scanning/
          │        │  │                      │     │   │           ├─ a
          │        │  │                      │     │   │           │  ├─ lerts [295]
          │        │  │                      │     │   │           │  │      ╰─ /
          │        │  │                      │     │   │           │  │         ╰─ {alert_number} [296]
          │        │  │                      │     │   │           │  │                         ╰─ /instances [297]
          │        │  │                      │     │   │           │  ╰─ nalyses [298]
          │        │  │                      │     │   │           │           ╰─ /
          │        │  │                      │     │   │           │              ╰─ {analysis_id} [299]
          │        │  │                      │     │   │           ├─ codeql/
          │        │  │                      │     │   │           │        ├─ databases [300]
          │        │  │                      │     │   │           │        │          ╰─ /
          │        │  │                      │     │   │           │        │             ╰─ {language} [301]
          │        │  │                      │     │   │           │        ╰─ variant-analyses [302]
          │        │  │                      │     │   │           │                          ╰─ /
          │        │  │                      │     │   │           │                             ╰─ {codeql_variant_analysis_id} [303]
          │        │  │                      │     │   │           │                                                           ╰─ /repos/
          │        │  │                      │     │   │           │                                                                    ╰─ {repo_owner}
          │        │  │                      │     │   │           │                                                                                  ╰─ /
          │        │  │                      │     │   │           │                                                                                     ╰─ {repo_name} [304]
          │        │  │                      │     │   │           ├─ default-setup [305]
          │        │  │                      │     │   │           ╰─ sarifs [306]
          │        │  │                      │     │   │                   ╰─ /
          │        │  │                      │     │   │                      ╰─ {sarif_id} [307]
          │        │  │                      │     │   ├─ owners/errors [308]
          │        │  │                      │     │   ╰─ spaces [309]
          │        │  │                      │     │           ╰─ /
          │        │  │                      │     │              ├─ devcontainers [310]
          │        │  │                      │     │              ├─ machines [311]
          │        │  │                      │     │              ├─ new [312]
          │        │  │                      │     │              ├─ permissions_check [313]
          │        │  │                      │     │              ╰─ secrets [314]
          │        │  │                      │     │                       ╰─ /
          │        │  │                      │     │                          ├─ public-key [315]
          │        │  │                      │     │                          ╰─ {secret_name} [316]
          │        │  │                      │     ├─ llaborators [317]
          │        │  │                      │     │            ╰─ /
          │        │  │                      │     │               ╰─ {username} [318]
          │        │  │                      │     │                           ╰─ /permission [319]
          │        │  │                      │     ├─ m
          │        │  │                      │     │  ├─ m
          │        │  │                      │     │  │  ├─ ents [320]
          │        │  │                      │     │  │  │     ╰─ /
          │        │  │                      │     │  │  │        ╰─ {comment_id} [321]
          │        │  │                      │     │  │  │                      ╰─ /reactions [322]
          │        │  │                      │     │  │  │                                  ╰─ /
          │        │  │                      │     │  │  │                                     ╰─ {reaction_id} [323]
          │        │  │                      │     │  │  ├─ its [324]
          │        │  │                      │     │  │  │    ╰─ /
          │        │  │                      │     │  │  │       ├─ {commit_sha}
          │        │  │                      │     │  │  │       │             ╰─ /
          │        │  │                      │     │  │  │       │                ├─ branches-where-head [325]
          │        │  │                      │     │  │  │       │                ├─ comments [326]
          │        │  │                      │     │  │  │       │                ╰─ pulls [327]
          │        │  │                      │     │  │  │       ╰─ {ref} [328]
          │        │  │                      │     │  │  │              ╰─ /
          │        │  │                      │     │  │  │                 ├─ check-
          │        │  │                      │     │  │  │                 │       ├─ runs [329]
          │        │  │                      │     │  │  │                 │       ╰─ suites [330]
          │        │  │                      │     │  │  │                 ╰─ status [331]
          │        │  │                      │     │  │  │                         ╰─ es [332]
          │        │  │                      │     │  │  ╰─ unity/profile [333]
          │        │  │                      │     │  ╰─ pare/
          │        │  │                      │     │         ╰─ {basehead} [334]
          │        │  │                      │     ╰─ nt
          │        │  │                      │         ├─ ents/
          │        │  │                      │         │      ╰─ {path} [335]
          │        │  │                      │         ╰─ ributors [336]
          │        │  │                      ├─ d
          │        │  │                      │  ├─ ep
          │        │  │                      │  │   ├─ end
          │        │  │                      │  │   │    ├─ abot/
          │        │  │                      │  │   │    │      ├─ alerts [337]
          │        │  │                      │  │   │    │      │       ╰─ /
          │        │  │                      │  │   │    │      │          ╰─ {alert_number} [338]
          │        │  │                      │  │   │    │      ╰─ secrets [339]
          │        │  │                      │  │   │    │               ╰─ /
          │        │  │                      │  │   │    │                  ├─ public-key [340]
          │        │  │                      │  │   │    │                  ╰─ {secret_name} [341]
          │        │  │                      │  │   │    ╰─ ency-graph/
          │        │  │                      │  │   │                 ├─ compare/
          │        │  │                      │  │   │                 │         ╰─ {basehead} [342]
          │        │  │                      │  │   │                 ╰─ s
          │        │  │                      │  │   │                    ├─ bom [343]
          │        │  │                      │  │   │                    ╰─ napshots [344]
          │        │  │                      │  │   ╰─ loyments [345]
          │        │  │                      │  │             ╰─ /
          │        │  │                      │  │                ╰─ {deployment_id} [346]
          │        │  │                      │  │                                 ╰─ /statuses [347]
          │        │  │                      │  │                                            ╰─ /
          │        │  │                      │  │                                               ╰─ {status_id} [348]
          │        │  │                      │  ╰─ ispatches [349]
          │        │  │                      ├─ e
          │        │  │                      │  ├─ nvironments [350]
          │        │  │                      │  │            ╰─ /
          │        │  │                      │  │               ╰─ {environment_name} [351]
          │        │  │                      │  │                                   ╰─ /
          │        │  │                      │  │                                      ├─ deployment
          │        │  │                      │  │                                      │           ├─ -branch-policies [352]
          │        │  │                      │  │                                      │           │                 ╰─ /
          │        │  │                      │  │                                      │           │                    ╰─ {branch_policy_id} [353]
          │        │  │                      │  │                                      │           ╰─ _protection_rules [354]
          │        │  │                      │  │                                      │                              ╰─ /
          │        │  │                      │  │                                      │                                 ├─ apps [355]
          │        │  │                      │  │                                      │                                 ╰─ {protection_rule_id} [356]
          │        │  │                      │  │                                      ├─ secrets [357]
          │        │  │                      │  │                                      │        ╰─ /
          │        │  │                      │  │                                      │           ├─ public-key [358]
          │        │  │                      │  │                                      │           ╰─ {secret_name} [359]
          │        │  │                      │  │                                      ╰─ variables [360]
          │        │  │                      │  │                                                 ╰─ /
          │        │  │                      │  │                                                    ╰─ {name} [361]
          │        │  │                      │  ╰─ vents [362]
          │        │  │                      ├─ forks [363]
          │        │  │                      ├─ git/
          │        │  │                      │     ├─ blobs [364]
          │        │  │                      │     │      ╰─ /
          │        │  │                      │     │         ╰─ {file_sha} [365]
          │        │  │                      │     ├─ commits [366]
          │        │  │                      │     │        ╰─ /
          │        │  │                      │     │           ╰─ {commit_sha} [367]
          │        │  │                      │     ├─ matching-refs/
          │        │  │                      │     │               ╰─ {ref} [368]
          │        │  │                      │     ├─ ref
          │        │  │                      │     │    ├─ /
          │        │  │                      │     │    │  ╰─ {ref} [369]
          │        │  │                      │     │    ╰─ s [370]
          │        │  │                      │     │       ╰─ /
          │        │  │                      │     │          ╰─ {ref} [371]
          │        │  │                      │     ╰─ t
          │        │  │                      │        ├─ ags [372]
          │        │  │                      │        │    ╰─ /
          │        │  │                      │        │       ╰─ {tag_sha} [373]
          │        │  │                      │        ╰─ rees [374]
          │        │  │                      │              ╰─ /
          │        │  │                      │                 ╰─ {tree_sha} [375]
          │        │  │                      ├─ hooks [376]
          │        │  │                      │      ╰─ /
          │        │  │                      │         ╰─ {hook_id} [377]
          │        │  │                      │                    ╰─ /
          │        │  │                      │                       ├─ config [378]
          │        │  │                      │                       ├─ deliveries [379]
          │        │  │                      │                       │           ╰─ /
          │        │  │                      │                       │              ╰─ {delivery_id} [380]
          │        │  │                      │                       │                             ╰─ /attempts [381]
          │        │  │                      │                       ├─ pings [382]
          │        │  │                      │                       ╰─ tests [383]
          │        │  │                      ├─ i
          │        │  │                      │  ├─ mport [384]
          │        │  │                      │  │      ╰─ /
          │        │  │                      │  │         ├─ authors [385]
          │        │  │                      │  │         │        ╰─ /
          │        │  │                      │  │         │           ╰─ {author_id} [386]
          │        │  │                      │  │         ╰─ l
          │        │  │                      │  │            ├─ arge_files [387]
          │        │  │                      │  │            ╰─ fs [388]
          │        │  │                      │  ├─ n
          │        │  │                      │  │  ├─ stallation [389]
          │        │  │                      │  │  ├─ teraction-limits [390]
          │        │  │                      │  │  ╰─ vitations [391]
          │        │  │                      │  │             ╰─ /
          │        │  │                      │  │                ╰─ {invitation_id} [392]
          │        │  │                      │  ╰─ ssues [393]
          │        │  │                      │         ╰─ /
          │        │  │                      │            ├─ comments [394]
          │        │  │                      │            │         ╰─ /
          │        │  │                      │            │            ╰─ {comment_id} [395]
          │        │  │                      │            │                          ╰─ /reactions [396]
          │        │  │                      │            │                                      ╰─ /
          │        │  │                      │            │                                         ╰─ {reaction_id} [397]
          │        │  │                      │            ├─ events [398]
          │        │  │                      │            │       ╰─ /
          │        │  │                      │            │          ╰─ {event_id} [399]
          │        │  │                      │            ╰─ {issue_number} [400]
          │        │  │                      │                            ╰─ /
          │        │  │                      │                               ├─ assignees [401]
          │        │  │                      │                               │          ╰─ /
          │        │  │                      │                               │             ╰─ {assignee} [402]
          │        │  │                      │                               ├─ comments [403]
          │        │  │                      │                               ├─ events [404]
          │        │  │                      │                               ├─ l
          │        │  │                      │                               │  ├─ abels [405]
          │        │  │                      │                               │  │      ╰─ /
          │        │  │                      │                               │  │         ╰─ {name} [406]
          │        │  │                      │                               │  ╰─ ock [407]
          │        │  │                      │                               ├─ reactions [408]
          │        │  │                      │                               │          ╰─ /
          │        │  │                      │                               │             ╰─ {reaction_id} [409]
          │        │  │                      │                               ╰─ timeline [410]
          │        │  │                      ├─ keys [411]
          │        │  │                      │     ╰─ /
          │        │  │                      │        ╰─ {key_id} [412]
          │        │  │                      ├─ l
          │        │  │                      │  ├─ a
          │        │  │                      │  │  ├─ bels [413]
          │        │  │                      │  │  │     ╰─ /
          │        │  │                      │  │  │        ╰─ {name} [414]
          │        │  │                      │  │  ╰─ nguages [415]
          │        │  │                      │  ╰─ icense [416]
          │        │  │                      ├─ m
          │        │  │                      │  ├─ erge
          │        │  │                      │  │     ├─ -upstream [417]
          │        │  │                      │  │     ╰─ s [418]
          │        │  │                      │  ╰─ ilestones [419]
          │        │  │                      │             ╰─ /
          │        │  │                      │                ╰─ {milestone_number} [420]
          │        │  │                      │                                    ╰─ /labels [421]
          │        │  │                      ├─ notifications [422]
          │        │  │                      ├─ p
          │        │  │                      │  ├─ ages [423]
          │        │  │                      │  │     ╰─ /
          │        │  │                      │  │        ├─ builds [424]
          │        │  │                      │  │        │       ╰─ /
          │        │  │                      │  │        │          ├─ latest [425]
          │        │  │                      │  │        │          ╰─ {build_id} [426]
          │        │  │                      │  │        ├─ deployments [427]
          │        │  │                      │  │        │            ╰─ /
          │        │  │                      │  │        │               ╰─ {pages_deployment_id} [428]
          │        │  │                      │  │        │                                      ╰─ /cancel [429]
          │        │  │                      │  │        ╰─ health [430]
          │        │  │                      │  ├─ r
          │        │  │                      │  │  ├─ ivate-vulnerability-reporting [431]
          │        │  │                      │  │  ╰─ o
          │        │  │                      │  │     ├─ jects [432]
          │        │  │                      │  │     ╰─ perties/values [433]
          │        │  │                      │  ╰─ ulls [434]
          │        │  │                      │        ╰─ /
          │        │  │                      │           ├─ comments [435]
          │        │  │                      │           │         ╰─ /
          │        │  │                      │           │            ╰─ {comment_id} [436]
          │        │  │                      │           │                          ╰─ /reactions [437]
          │        │  │                      │           │                                      ╰─ /
          │        │  │                      │           │                                         ╰─ {reaction_id} [438]
          │        │  │                      │           ╰─ {pull_number} [439]
          │        │  │                      │                          ╰─ /
          │        │  │                      │                             ├─ co
          │        │  │                      │                             │   ├─ despaces [440]
          │        │  │                      │                             │   ╰─ mm
          │        │  │                      │                             │       ├─ ents [441]
          │        │  │                      │                             │       │     ╰─ /
          │        │  │                      │                             │       │        ╰─ {comment_id}
          │        │  │                      │                             │       │                      ╰─ /replies [442]
          │        │  │                      │                             │       ╰─ its [443]
          │        │  │                      │                             ├─ files [444]
          │        │  │                      │                             ├─ merge [445]
          │        │  │                      │                             ├─ re
          │        │  │                      │                             │   ├─ quested_reviewers [446]
          │        │  │                      │                             │   ╰─ views [447]
          │        │  │                      │                             │          ╰─ /
          │        │  │                      │                             │             ╰─ {review_id} [448]
          │        │  │                      │                             │                          ╰─ /
          │        │  │                      │                             │                             ├─ comments [449]
          │        │  │                      │                             │                             ├─ dismissals [450]
          │        │  │                      │                             │                             ╰─ events [451]
          │        │  │                      │                             ╰─ update-branch [452]
          │        │  │                      ├─ r
          │        │  │                      │  ├─ e
          │        │  │                      │  │  ├─ adme [453]
          │        │  │                      │  │  │     ╰─ /
          │        │  │                      │  │  │        ╰─ {dir} [454]
          │        │  │                      │  │  ╰─ leases [455]
          │        │  │                      │  │          ╰─ /
          │        │  │                      │  │             ├─ assets/
          │        │  │                      │  │             │        ╰─ {asset_id} [456]
          │        │  │                      │  │             ├─ generate-notes [457]
          │        │  │                      │  │             ├─ latest [458]
          │        │  │                      │  │             ├─ tags/
          │        │  │                      │  │             │      ╰─ {tag} [459]
          │        │  │                      │  │             ╰─ {release_id} [460]
          │        │  │                      │  │                           ╰─ /
          │        │  │                      │  │                              ├─ assets [461]
          │        │  │                      │  │                              ╰─ reactions [462]
          │        │  │                      │  │                                         ╰─ /
          │        │  │                      │  │                                            ╰─ {reaction_id} [463]
          │        │  │                      │  ╰─ ules
          │        │  │                      │        ├─ /branches/
          │        │  │                      │        │           ╰─ {branch} [464]
          │        │  │                      │        ╰─ ets [465]
          │        │  │                      │             ╰─ /
          │        │  │                      │                ├─ rule-suites [466]
          │        │  │                      │                │            ╰─ /
          │        │  │                      │                │               ╰─ {rule_suite_id} [467]
          │        │  │                      │                ╰─ {ruleset_id} [468]
          │        │  │                      ├─ s
          │        │  │                      │  ├─ ec
          │        │  │                      │  │   ├─ ret-scanning/alerts [469]
          │        │  │                      │  │   │                    ╰─ /
          │        │  │                      │  │   │                       ╰─ {alert_number} [470]
          │        │  │                      │  │   │                                       ╰─ /locations [471]
          │        │  │                      │  │   ╰─ urity-advisories [472]
          │        │  │                      │  │                     ╰─ /
          │        │  │                      │  │                        ├─ reports [473]
          │        │  │                      │  │                        ╰─ {ghsa_id} [474]
          │        │  │                      │  │                                   ╰─ /
          │        │  │                      │  │                                      ├─ cve [475]
          │        │  │                      │  │                                      ╰─ forks [476]
          │        │  │                      │  ├─ ta
          │        │  │                      │  │   ├─ rgazers [477]
          │        │  │                      │  │   ╰─ t
          │        │  │                      │  │      ├─ s/
          │        │  │                      │  │      │   ├─ co
          │        │  │                      │  │      │   │   ├─ de_frequency [478]
          │        │  │                      │  │      │   │   ├─ mmit_activity [479]
          │        │  │                      │  │      │   │   ╰─ ntributors [480]
          │        │  │                      │  │      │   ╰─ p
          │        │  │                      │  │      │      ├─ articipation [481]
          │        │  │                      │  │      │      ╰─ unch_card [482]
          │        │  │                      │  │      ╰─ uses/
          │        │  │                      │  │             ╰─ {sha} [483]
          │        │  │                      │  ╰─ ubscri
          │        │  │                      │          ├─ bers [484]
          │        │  │                      │          ╰─ ption [485]
          │        │  │                      ├─ t
          │        │  │                      │  ├─ a
          │        │  │                      │  │  ├─ gs [486]
          │        │  │                      │  │  │   ╰─ /protection [487]
          │        │  │                      │  │  │                ╰─ /
          │        │  │                      │  │  │                   ╰─ {tag_protection_id} [488]
          │        │  │                      │  │  ╰─ rball/
          │        │  │                      │  │          ╰─ {ref} [489]
          │        │  │                      │  ├─ eams [490]
          │        │  │                      │  ├─ opics [491]
          │        │  │                      │  ╰─ ra
          │        │  │                      │      ├─ ffic/
          │        │  │                      │      │      ├─ clones [492]
          │        │  │                      │      │      ├─ popular/
          │        │  │                      │      │      │         ├─ paths [493]
          │        │  │                      │      │      │         ╰─ referrers [494]
          │        │  │                      │      │      ╰─ views [495]
          │        │  │                      │      ╰─ nsfer [496]
          │        │  │                      ├─ vulnerability-alerts [497]
          │        │  │                      ╰─ zipball/
          │        │  │                                ╰─ {ref} [498]
          │        │  ╰─ {template_owner}
          │        │                    ╰─ /
          │        │                       ╰─ {template_repo}
          │        │                                        ╰─ /generate [499]
          │        ╰─ itories [500]
          ├─ search/
          │        ├─ co
          │        │   ├─ de [501]
          │        │   ╰─ mmits [502]
          │        ├─ issues [503]
          │        ├─ labels [504]
          │        ├─ repositories [505]
          │        ├─ topics [506]
          │        ╰─ users [507]
          ├─ teams/
          │       ╰─ {team_id} [508]
          │                  ╰─ /
          │                     ├─ discussions [509]
          │                     │            ╰─ /
          │                     │               ╰─ {discussion_number} [510]
          │                     │                                    ╰─ /
          │                     │                                       ├─ comments [511]
          │                     │                                       │         ╰─ /
          │                     │                                       │            ╰─ {comment_number} [512]
          │                     │                                       │                              ╰─ /reactions [513]
          │                     │                                       ╰─ reactions [514]
          │                     ├─ invitations [515]
          │                     ├─ members [516]
          │                     │        ├─ /
          │                     │        │  ╰─ {username} [517]
          │                     │        ╰─ hips/
          │                     │               ╰─ {username} [518]
          │                     ├─ projects [519]
          │                     │         ╰─ /
          │                     │            ╰─ {project_id} [520]
          │                     ├─ repos [521]
          │                     │      ╰─ /
          │                     │         ╰─ {owner}
          │                     │                  ╰─ /
          │                     │                     ╰─ {repo} [522]
          │                     ╰─ teams [523]
          ├─ user [524]
          │     ├─ /
          │     │  ├─ blocks [525]
          │     │  │       ╰─ /
          │     │  │          ╰─ {username} [526]
          │     │  ├─ codespaces [527]
          │     │  │           ╰─ /
          │     │  │              ├─ secrets [528]
          │     │  │              │        ╰─ /
          │     │  │              │           ├─ public-key [529]
          │     │  │              │           ╰─ {secret_name} [530]
          │     │  │              │                          ╰─ /repositories [531]
          │     │  │              │                                         ╰─ /
          │     │  │              │                                            ╰─ {repository_id} [532]
          │     │  │              ╰─ {codespace_name} [533]
          │     │  │                                ╰─ /
          │     │  │                                   ├─ exports [534]
          │     │  │                                   │        ╰─ /
          │     │  │                                   │           ╰─ {export_id} [535]
          │     │  │                                   ├─ machines [536]
          │     │  │                                   ├─ publish [537]
          │     │  │                                   ╰─ st
          │     │  │                                       ├─ art [538]
          │     │  │                                       ╰─ op [539]
          │     │  ├─ docker/conflicts [540]
          │     │  ├─ email
          │     │  │      ├─ /visibility [541]
          │     │  │      ╰─ s [542]
          │     │  ├─ follow
          │     │  │       ├─ ers [543]
          │     │  │       ╰─ ing [544]
          │     │  │            ╰─ /
          │     │  │               ╰─ {username} [545]
          │     │  ├─ gpg_keys [546]
          │     │  │         ╰─ /
          │     │  │            ╰─ {gpg_key_id} [547]
          │     │  ├─ i
          │     │  │  ├─ n
          │     │  │  │  ├─ stallations [548]
          │     │  │  │  │            ╰─ /
          │     │  │  │  │               ╰─ {installation_id}
          │     │  │  │  │                                  ╰─ /repositories [549]
          │     │  │  │  │                                                 ╰─ /
          │     │  │  │  │                                                    ╰─ {repository_id} [550]
          │     │  │  │  ╰─ teraction-limits [551]
          │     │  │  ╰─ ssues [552]
          │     │  ├─ keys [553]
          │     │  │     ╰─ /
          │     │  │        ╰─ {key_id} [554]
          │     │  ├─ m
          │     │  │  ├─ arketplace_purchases [555]
          │     │  │  │                     ╰─ /stubbed [556]
          │     │  │  ├─ emberships/orgs [557]
          │     │  │  │                ╰─ /
          │     │  │  │                   ╰─ {org} [558]
          │     │  │  ╰─ igrations [559]
          │     │  │             ╰─ /
          │     │  │                ╰─ {migration_id} [560]
          │     │  │                                ╰─ /
          │     │  │                                   ├─ archive [561]
          │     │  │                                   ╰─ repos
          │     │  │                                          ├─ /
          │     │  │                                          │  ╰─ {repo_name}
          │     │  │                                          │               ╰─ /lock [562]
          │     │  │                                          ╰─ itories [563]
          │     │  ├─ orgs [564]
          │     │  ├─ p
          │     │  │  ├─ ackages [565]
          │     │  │  │        ╰─ /
          │     │  │  │           ╰─ {package_type}
          │     │  │  │                           ╰─ /
          │     │  │  │                              ╰─ {package_name} [566]
          │     │  │  │                                              ╰─ /
          │     │  │  │                                                 ├─ restore [567]
          │     │  │  │                                                 ╰─ versions [568]
          │     │  │  │                                                           ╰─ /
          │     │  │  │                                                              ╰─ {package_version_id} [569]
          │     │  │  │                                                                                    ╰─ /restore [570]
          │     │  │  ├─ rojects [571]
          │     │  │  ╰─ ublic_emails [572]
          │     │  ├─ repos [573]
          │     │  │      ╰─ itory_invitations [574]
          │     │  │                         ╰─ /
          │     │  │                            ╰─ {invitation_id} [575]
          │     │  ├─ s
          │     │  │  ├─ ocial_accounts [576]
          │     │  │  ├─ sh_signing_keys [577]
          │     │  │  │                ╰─ /
          │     │  │  │                   ╰─ {ssh_signing_key_id} [578]
          │     │  │  ├─ tarred [579]
          │     │  │  │       ╰─ /
          │     │  │  │          ╰─ {owner}
          │     │  │  │                   ╰─ /
          │     │  │  │                      ╰─ {repo} [580]
          │     │  │  ╰─ ubscriptions [581]
          │     │  ╰─ teams [582]
          │     ╰─ s [583]
          │        ╰─ /
          │           ╰─ {username} [584]
          │                       ╰─ /
          │                          ├─ docker/conflicts [585]
          │                          ├─ events [586]
          │                          │       ╰─ /
          │                          │          ├─ orgs/
          │                          │          │      ╰─ {org} [587]
          │                          │          ╰─ public [588]
          │                          ├─ follow
          │                          │       ├─ ers [589]
          │                          │       ╰─ ing [590]
          │                          │            ╰─ /
          │                          │               ╰─ {target_user} [591]
          │                          ├─ g
          │                          │  ├─ ists [592]
          │                          │  ╰─ pg_keys [593]
          │                          ├─ hovercard [594]
          │                          ├─ installation [595]
          │                          ├─ keys [596]
          │                          ├─ orgs [597]
          │                          ├─ p
          │                          │  ├─ ackages [598]
          │                          │  │        ╰─ /
          │                          │  │           ╰─ {package_type}
          │                          │  │                           ╰─ /
          │                          │  │                              ╰─ {package_name} [599]
          │                          │  │                                              ╰─ /
          │                          │  │                                                 ├─ restore [600]
          │                          │  │                                                 ╰─ versions [601]
          │                          │  │                                                           ╰─ /
          │                          │  │                                                              ╰─ {package_version_id} [602]
          │                          │  │                                                                                    ╰─ /restore [603]
          │                          │  ╰─ rojects [604]
          │                          ├─ re
          │                          │   ├─ ceived_events [605]
          │                          │   │              ╰─ /public [606]
          │                          │   ╰─ pos [607]
          │                          ╰─ s
          │                             ├─ ettings/billing/
          │                             │                 ├─ actions [608]
          │                             │                 ├─ packages [609]
          │                             │                 ╰─ shared-storage [610]
          │                             ├─ ocial_accounts [611]
          │                             ├─ sh_signing_keys [612]
          │                             ├─ tarred [613]
          │                             ╰─ ubscriptions [614]
          ├─ versions [615]
          ╰─ zen [616]
    "###);
}
