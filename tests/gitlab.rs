use router::trie::Trie;

#[path = "../fixtures/gitlab.rs"]
mod gitlab;

#[allow(clippy::too_many_lines)]
#[test]
fn test() {
    let mut trie = Trie::new();
    let mut counter = 0;
    for route in gitlab::ROUTES {
        counter += 1;
        trie.insert(route, counter);
    }

    insta::assert_snapshot!(trie, @r###"
    ROOT
       ╰─ / [1472]
          ├─ -/
          │   ├─ a
          │   │  ├─ buse_reports [1]
          │   │  │             ╰─ /add_category [2]
          │   │  ├─ cme-challenge [3]
          │   │  ╰─ utocomplete/
          │   │                ├─ award_emojis [221]
          │   │                ├─ deploy_keys_with_owners [222]
          │   │                ├─ group_subgroups [223]
          │   │                ├─ merge_request_
          │   │                │               ├─ source_branches [224]
          │   │                │               ╰─ target_branches [225]
          │   │                ├─ namespace_routes [226]
          │   │                ├─ project
          │   │                │        ├─ _
          │   │                │        │  ├─ groups [227]
          │   │                │        │  ╰─ routes [228]
          │   │                │        ╰─ s [229]
          │   │                ╰─ users [230]
          │   │                       ╰─ /
          │   │                          ╰─ {id} [231]
          │   ├─ c
          │   │  ├─ haos/
          │   │  │      ├─ cpu_spin [233]
          │   │  │      ├─ db_spin [234]
          │   │  │      ├─ gc [235]
          │   │  │      ├─ kill [236]
          │   │  │      ├─ leakmem [237]
          │   │  │      ├─ quit [238]
          │   │  │      ╰─ sleep [239]
          │   │  ├─ ountr
          │   │  │      ├─ ies [248]
          │   │  │      ╰─ y_states [249]
          │   │  ╰─ ustomers_dot/proxy/graphql [250]
          │   ├─ kubernetes [240]
          │   │           ╰─ /
          │   │              ╰─ {agent_id} [241]
          │   │                          ╰─ /
          │   │                             ╰─ {vueroute:*} [242]
          │   ├─ p
          │   │  ├─ rofile [1554]
          │   │  │       ╰─ /
          │   │  │          ├─ emails [749]
          │   │  │          │       ╰─ /
          │   │  │          │          ├─ confirmation [243]
          │   │  │          │          │             ╰─ /new [244]
          │   │  │          │          ╰─ {id} [750]
          │   │  │          │                ╰─ /resend_confirmation_instructions [751]
          │   │  │          ├─ reset_
          │   │  │          │       ├─ feed_token [735]
          │   │  │          │       ├─ incoming_email_token [736]
          │   │  │          │       ╰─ static_object_token [737]
          │   │  │          ├─ u
          │   │  │          │  ├─ pdate_username [738]
          │   │  │          │  ╰─ sage_quotas [761]
          │   │  │          ├─ a
          │   │  │          │  ├─ c
          │   │  │          │  │  ├─ count [739]
          │   │  │          │  │  │      ╰─ /unlink [740]
          │   │  │          │  │  ╰─ tive_sessions/
          │   │  │          │  │                  ╰─ {id} [1538]
          │   │  │          │  ╰─ vatar [741]
          │   │  │          ├─ billings [742]
          │   │  │          ├─ c
          │   │  │          │  ├─ hat_names [743]
          │   │  │          │  │          ╰─ /
          │   │  │          │  │             ├─ deny [745]
          │   │  │          │  │             ├─ new [746]
          │   │  │          │  │             ╰─ {id} [744]
          │   │  │          │  ╰─ omment_templates [747]
          │   │  │          │                    ╰─ /
          │   │  │          │                       ╰─ {id} [748]
          │   │  │          ├─ groups/
          │   │  │          │        ╰─ {id:*}
          │   │  │          │                ╰─ /notifications [752]
          │   │  │          ├─ notifications [753]
          │   │  │          ├─ p
          │   │  │          │  ├─ references [754]
          │   │  │          │  ╰─ ersonal_access_tokens/
          │   │  │          │                          ╰─ {id}
          │   │  │          │                                ╰─ /revoke [1551]
          │   │  │          ├─ slack/
          │   │  │          │       ├─ edit [755]
          │   │  │          │       ╰─ slack_link [756]
          │   │  │          ├─ two_factor_auth [757]
          │   │  │          │                ╰─ /
          │   │  │          │                   ├─ c
          │   │  │          │                   │  ├─ odes [758]
          │   │  │          │                   │  ╰─ reate_webauthn [759]
          │   │  │          │                   ╰─ skip [760]
          │   │  │          ╰─ webauthn_registrations/
          │   │  │                                   ╰─ {id} [762]
          │   │  ├─ hone_verification/telesign_callback [734]
          │   │  ╰─ ush_from_secondary/
          │   │                       ╰─ {geo_node_id}
          │   │                                      ╰─ /
          │   │                                         ╰─ {repository_path:*}
          │   │                                                              ╰─ /
          │   │                                                                 ├─ git
          │   │                                                                 │    ├─ -
          │   │                                                                 │    │  ├─ receive-pack [1440]
          │   │                                                                 │    │  ╰─ upload-pack [1441]
          │   │                                                                 │    ╰─ lab-lfs/objects/
          │   │                                                                 │                      ├─ {oid:*}
          │   │                                                                 │                      │        ╰─ /
          │   │                                                                 │                      │           ├─ {size:*}
          │   │                                                                 │                      │           │         ╰─ /authorize [1468]
          │   │                                                                 │                      │           ╰─ {size:*} [1467]
          │   │                                                                 │                      ╰─ {oid:*} [1466]
          │   │                                                                 ├─ info/
          │   │                                                                 │      ├─ refs [1442]
          │   │                                                                 │      ╰─ lfs/
          │   │                                                                 │            ├─ objects [1448]
          │   │                                                                 │            │        ╰─ /
          │   │                                                                 │            │           ├─ batch [1450]
          │   │                                                                 │            │           ╰─ {oid:*} [1449]
          │   │                                                                 │            ╰─ locks [1454]
          │   │                                                                 │                   ╰─ /
          │   │                                                                 │                      ├─ new [1458]
          │   │                                                                 │                      ├─ verify [1459]
          │   │                                                                 │                      ╰─ {id} [1455]
          │   │                                                                 │                            ╰─ /
          │   │                                                                 │                               ├─ edit [1456]
          │   │                                                                 │                               ╰─ unlock [1457]
          │   │                                                                 ╰─ ssh-upload-pack [1443]
          │   ├─ external_redirect [283]
          │   ├─ google_api/auth/callback [284]
          │   ├─ liveness [587]
          │   ├─ r
          │   │  ├─ e
          │   │  │  ├─ adiness [588]
          │   │  │  ╰─ mote_development/workspaces [1427]
          │   │  │                               ├─ /
          │   │  │                               │  ├─ new [1438]
          │   │  │                               │  ├─ {id} [1434]
          │   │  │                               │  │     ╰─ /edit [1435]
          │   │  │                               │  ├─ {workspace_id}
          │   │  │                               │  │               ╰─ /workspaces [1436]
          │   │  │                               │  │                            ╰─ /new [1437]
          │   │  │                               │  ├─ {vueroute:*}
          │   │  │                               │  │             ╰─ /
          │   │  │                               │  │                ├─ new [1433]
          │   │  │                               │  │                ├─ {id} [1429]
          │   │  │                               │  │                │     ╰─ /edit [1430]
          │   │  │                               │  │                ╰─ {workspace_id}
          │   │  │                               │  │                                ╰─ /workspaces [1431]
          │   │  │                               │  │                                             ╰─ /new [1432]
          │   │  │                               │  ╰─ {vueroute:*} [1428]
          │   │  │                               ╰─ _feature_flag [1439]
          │   │  ╰─ unner_setup/platforms [1473]
          │   ├─ i
          │   │  ├─ de [597]
          │   │  │   ├─ /
          │   │  │   │  ├─ oauth_redirect [598]
          │   │  │   │  ├─ project [599]
          │   │  │   │  │        ╰─ /
          │   │  │   │  │           ╰─ {project_id} [600]
          │   │  │   │  │                         ╰─ /
          │   │  │   │  │                            ├─ blob [601]
          │   │  │   │  │                            │     ╰─ /
          │   │  │   │  │                            │        ├─ {branch:*}
          │   │  │   │  │                            │        │           ╰─ /- [603]
          │   │  │   │  │                            │        │               ╰─ /
          │   │  │   │  │                            │        │                  ╰─ {path:*} [604]
          │   │  │   │  │                            │        ╰─ {branch:*} [602]
          │   │  │   │  │                            ├─ edit [605]
          │   │  │   │  │                            │     ╰─ /
          │   │  │   │  │                            │        ├─ {branch:*}
          │   │  │   │  │                            │        │           ╰─ /- [607]
          │   │  │   │  │                            │        │               ╰─ /
          │   │  │   │  │                            │        │                  ╰─ {path:*} [608]
          │   │  │   │  │                            │        ╰─ {branch:*} [606]
          │   │  │   │  │                            ├─ merge_requests/
          │   │  │   │  │                            │                ╰─ {merge_request_id} [609]
          │   │  │   │  │                            ╰─ tree [610]
          │   │  │   │  │                                  ╰─ /
          │   │  │   │  │                                     ├─ {branch:*}
          │   │  │   │  │                                     │           ╰─ /- [612]
          │   │  │   │  │                                     │               ╰─ /
          │   │  │   │  │                                     │                  ╰─ {path:*} [613]
          │   │  │   │  │                                     ╰─ {branch:*} [611]
          │   │  │   │  ╰─ remote/
          │   │  │   │           ╰─ {remote_host} [1609]
          │   │  │   │                          ╰─ /
          │   │  │   │                             ╰─ {remote_path:*} [1610]
          │   │  │   ╰─ ntity_verification [1581]
          │   │  │                       ╰─ /
          │   │  │                          ├─ s
          │   │  │                          │  ├─ end_phone_verification_code [1582]
          │   │  │                          │  ╰─ uccess [1583]
          │   │  │                          ├─ toggle_phone_exemption [1584]
          │   │  │                          ╰─ verif
          │   │  │                                 ├─ ication_state [1585]
          │   │  │                                 ╰─ y_
          │   │  │                                     ├─ credit_card [1586]
          │   │  │                                     │            ╰─ _captcha [1587]
          │   │  │                                     ╰─ phone_verification_code [1588]
          │   │  ╰─ nvites/
          │   │           ╰─ {id} [670]
          │   │                 ╰─ /
          │   │                    ├─ accept [671]
          │   │                    ╰─ decline [672]
          │   ├─ j
          │   │  ├─ ira_connect/
          │   │  │             ├─ app_descriptor [673]
          │   │  │             ├─ branches/
          │   │  │             │          ├─ new [674]
          │   │  │             │          ╰─ route [675]
          │   │  │             ├─ events/
          │   │  │             │        ├─ installed [676]
          │   │  │             │        ╰─ uninstalled [677]
          │   │  │             ├─ installations [678]
          │   │  │             ├─ oauth_
          │   │  │             │       ├─ application_id [679]
          │   │  │             │       ╰─ callbacks [680]
          │   │  │             ├─ public_keys/
          │   │  │             │             ╰─ {id} [681]
          │   │  │             ├─ repositories/
          │   │  │             │              ├─ associate [682]
          │   │  │             │              ╰─ search [683]
          │   │  │             ├─ subscriptions [684]
          │   │  │             │              ╰─ /
          │   │  │             │                 ╰─ {id} [685]
          │   │  │             ╰─ workspaces/search [686]
          │   │  ╰─ wks [687]
          │   ├─ m
          │   │  ├─ a
          │   │  │  ├─ ilgun/webhooks [693]
          │   │  │  ╰─ nifest [1411]
          │   │  ╰─ e
          │   │     ├─ mbers/mailgun/permanent_failures [694]
          │   │     ╰─ trics [695]
          │   │            ╰─ /system [696]
          │   ├─ user
          │   │     ├─ _settings/
          │   │     │           ├─ a
          │   │     │           │  ├─ pplications [697]
          │   │     │           │  ├─ ctive_sessions [1539]
          │   │     │           │  │               ╰─ /
          │   │     │           │  │                  ├─ saml [1541]
          │   │     │           │  │                  ╰─ {id} [1540]
          │   │     │           │  ╰─ uthentication_log [1559]
          │   │     │           ├─ gpg_keys [1542]
          │   │     │           │         ╰─ /
          │   │     │           │            ╰─ {id} [1543]
          │   │     │           │                  ╰─ /revoke [1544]
          │   │     │           ├─ identities [1545]
          │   │     │           │           ╰─ /new [1546]
          │   │     │           ├─ p
          │   │     │           │  ├─ assword [1547]
          │   │     │           │  │        ╰─ /
          │   │     │           │  │           ├─ edit [1548]
          │   │     │           │  │           ├─ new [1549]
          │   │     │           │  │           ╰─ reset [1550]
          │   │     │           │  ├─ ersonal_access_tokens [1552]
          │   │     │           │  │                      ╰─ /
          │   │     │           │  │                         ╰─ {id}
          │   │     │           │  │                               ╰─ /revoke [1553]
          │   │     │           │  ╰─ rofile [1555]
          │   │     │           ╰─ ssh_keys [1556]
          │   │     │                     ╰─ /
          │   │     │                        ╰─ {id} [1557]
          │   │     │                              ╰─ /revoke [1558]
          │   │     ╰─ s/
          │   │         ├─ broadcast_message_dismissals [1578]
          │   │         ├─ callouts [1579]
          │   │         ├─ group_callouts [1580]
          │   │         ├─ p
          │   │         │  ├─ ins [1590]
          │   │         │  ╰─ roject_callouts [1591]
          │   │         ╰─ terms [1605]
          │   │                ╰─ /
          │   │                   ╰─ {id}
          │   │                         ╰─ /
          │   │                            ├─ accept [1606]
          │   │                            ╰─ decline [1607]
          │   ├─ o
          │   │  ├─ perations [717]
          │   │  │          ╰─ /environments [718]
          │   │  ├─ rganizations [722]
          │   │  │             ╰─ /
          │   │  │                ├─ new [727]
          │   │  │                ├─ preview_markdown [728]
          │   │  │                ╰─ {organization_path} [723]
          │   │  │                                     ╰─ /
          │   │  │                                        ├─ groups [719]
          │   │  │                                        │       ├─ /
          │   │  │                                        │       │  ├─ new [721]
          │   │  │                                        │       │  ╰─ {id:*}
          │   │  │                                        │       │          ╰─ /edit [720]
          │   │  │                                        │       ╰─ _and_projects [725]
          │   │  │                                        ├─ activity [724]
          │   │  │                                        ├─ users [726]
          │   │  │                                        ├─ projects/
          │   │  │                                        │          ╰─ {namespace_id:*}
          │   │  │                                        │                            ╰─ /
          │   │  │                                        │                               ╰─ {id}
          │   │  │                                        │                                     ╰─ /edit [729]
          │   │  │                                        ╰─ settings/general [730]
          │   │  ╰─ ffline [1412]
          │   ├─ s
          │   │  ├─ andbox/
          │   │  │        ├─ mermaid [1474]
          │   │  │        ╰─ swagger [1475]
          │   │  ├─ e
          │   │  │  ├─ curity/
          │   │  │  │        ├─ dashboard [1482]
          │   │  │  │        │          ╰─ /settings [1483]
          │   │  │  │        ├─ projects [1484]
          │   │  │  │        │         ╰─ /
          │   │  │  │        │            ╰─ {id} [1485]
          │   │  │  │        ╰─ vulnerabilities [1486]
          │   │  │  ╰─ nt_notifications/
          │   │  │                     ╰─ {id}
          │   │  │                           ╰─ /unsubscribe [1487]
          │   │  ├─ martcard/
          │   │  │          ├─ auth [1496]
          │   │  │          ├─ extract_certificate [1497]
          │   │  │          ╰─ verify_certificate [1498]
          │   │  ├─ nippets [1499]
          │   │  │        ╰─ /
          │   │  │           ├─ new [1505]
          │   │  │           ├─ preview_markdown [1506]
          │   │  │           ├─ {id} [1500]
          │   │  │           │     ╰─ /
          │   │  │           │        ├─ edit [1501]
          │   │  │           │        ├─ mark_as_spam [1502]
          │   │  │           │        ├─ raw [1503]
          │   │  │           │        ╰─ toggle_award_emoji [1504]
          │   │  │           ╰─ {snippet_id}
          │   │  │                         ╰─ /
          │   │  │                            ├─ raw/
          │   │  │                            │     ╰─ {ref}
          │   │  │                            │            ╰─ /
          │   │  │                            │               ╰─ {path:*} [1508]
          │   │  │                            ╰─ notes [1509]
          │   │  │                                   ╰─ /
          │   │  │                                      ╰─ {id} [1510]
          │   │  │                                            ╰─ /
          │   │  │                                               ├─ delete_attachment [1511]
          │   │  │                                               ╰─ toggle_award_emoji [1512]
          │   │  ╰─ ubscriptions [1513]
          │   │                ╰─ /
          │   │                   ├─ buy_
          │   │                   │     ├─ minutes [1514]
          │   │                   │     ╰─ storage [1515]
          │   │                   ├─ new [1516]
          │   │                   ├─ payment_
          │   │                   │         ├─ form [1517]
          │   │                   │         ╰─ method [1518]
          │   │                   ├─ validate_payment_method [1519]
          │   │                   ├─ groups [1520]
          │   │                   │       ╰─ /
          │   │                   │          ├─ new [1523]
          │   │                   │          ╰─ {id} [1521]
          │   │                   │                ╰─ /edit [1522]
          │   │                   ╰─ hand_raise_leads [1524]
          │   ├─ t
          │   │  ├─ r
          │   │  │  ├─ ial
          │   │  │  │    ├─ s [1525]
          │   │  │  │    │  ╰─ /
          │   │  │  │    │     ├─ new [1526]
          │   │  │  │    │     ╰─ duo_pro [1527]
          │   │  │  │    │              ╰─ /new [1528]
          │   │  │  │    ╰─ _registrations [1531]
          │   │  │  │                    ╰─ /new [1532]
          │   │  │  ╰─ ack_namespace_visits [1589]
          │   │  ╰─ imelogs [1530]
          │   ├─ whats_new [1612]
          │   ╰─ {model}
          │            ╰─ /
          │               ╰─ {model_id}
          │                           ╰─ /uploads/
          │                                      ╰─ {secret}
          │                                                ╰─ /
          │                                                   ╰─ {filename} [232]
          ├─ a
          │  ├─ dmin [87]
          │  │     ╰─ /
          │  │        ├─ a
          │  │        │  ├─ buse_reports [6]
          │  │        │  │             ╰─ /
          │  │        │  │                ╰─ {id} [7]
          │  │        │  │                      ╰─ /moderate_user [8]
          │  │        │  ├─ i/
          │  │        │  │   ├─ feature_settings [9]
          │  │        │  │   │                 ╰─ /
          │  │        │  │   │                    ╰─ {id} [10]
          │  │        │  │   │                          ╰─ /edit [11]
          │  │        │  │   ╰─ self_hosted_models [12]
          │  │        │  │                       ╰─ /
          │  │        │  │                          ├─ new [15]
          │  │        │  │                          ├─ terms_and_conditions [16]
          │  │        │  │                          ╰─ {id} [13]
          │  │        │  │                                ╰─ /edit [14]
          │  │        │  ├─ pplication
          │  │        │  │           ├─ _settings [17]
          │  │        │  │           │          ╰─ /
          │  │        │  │           │             ├─ a
          │  │        │  │           │             │  ├─ dvanced_search [18]
          │  │        │  │           │             │  ├─ nalytics [19]
          │  │        │  │           │             │  ╰─ ppearance [41]
          │  │        │  │           │             │             ╰─ /
          │  │        │  │           │             │                ├─ favicon [42]
          │  │        │  │           │             │                ├─ header_logos [43]
          │  │        │  │           │             │                ├─ logo [44]
          │  │        │  │           │             │                ╰─ p
          │  │        │  │           │             │                   ├─ review_sign_in [45]
          │  │        │  │           │             │                   ╰─ wa_icon [46]
          │  │        │  │           │             ├─ c
          │  │        │  │           │             │  ├─ i_cd [20]
          │  │        │  │           │             │  ╰─ lear_repository_check_states [21]
          │  │        │  │           │             ├─ ge
          │  │        │  │           │             │   ├─ neral [22]
          │  │        │  │           │             │   ╰─ o [107]
          │  │        │  │           │             ├─ integrations [23]
          │  │        │  │           │             │             ╰─ /
          │  │        │  │           │             │                ╰─ {id} [133]
          │  │        │  │           │             │                      ╰─ /
          │  │        │  │           │             │                         ├─ edit [134]
          │  │        │  │           │             │                         ├─ overrides [135]
          │  │        │  │           │             │                         ├─ reset [136]
          │  │        │  │           │             │                         ╰─ test [137]
          │  │        │  │           │             ├─ lets_encrypt_terms_of_service [24]
          │  │        │  │           │             ├─ metrics_and_profiling [25]
          │  │        │  │           │             ├─ n
          │  │        │  │           │             │  ├─ amespace_storage [26]
          │  │        │  │           │             │  ╰─ etwork [27]
          │  │        │  │           │             ├─ preferences [28]
          │  │        │  │           │             ├─ r
          │  │        │  │           │             │  ├─ e
          │  │        │  │           │             │  │  ├─ po
          │  │        │  │           │             │  │  │   ├─ rting [29]
          │  │        │  │           │             │  │  │   ╰─ sitory [30]
          │  │        │  │           │             │  │  ╰─ set_
          │  │        │  │           │             │  │        ├─ error_tracking_access_token [31]
          │  │        │  │           │             │  │        ├─ health_check_token [32]
          │  │        │  │           │             │  │        ╰─ registration_token [33]
          │  │        │  │           │             │  ╰─ oles_and_permissions [47]
          │  │        │  │           │             │                        ╰─ /
          │  │        │  │           │             │                           ├─ new [49]
          │  │        │  │           │             │                           ╰─ {id}
          │  │        │  │           │             │                                 ╰─ /edit [48]
          │  │        │  │           │             ├─ s
          │  │        │  │           │             │  ├─ e
          │  │        │  │           │             │  │  ├─ at_link_payload [34]
          │  │        │  │           │             │  │  ╰─ curity_and_compliance [35]
          │  │        │  │           │             │  ├─ lack [175]
          │  │        │  │           │             │  │     ├─ _app_manifest_
          │  │        │  │           │             │  │     │               ├─ download [36]
          │  │        │  │           │             │  │     │               ╰─ share [37]
          │  │        │  │           │             │  │     ╰─ /slack_auth [176]
          │  │        │  │           │             │  ╰─ cim_oauth [50]
          │  │        │  │           │             ├─ templates [38]
          │  │        │  │           │             ╰─ u
          │  │        │  │           │                ├─ pdate_microsoft_application [39]
          │  │        │  │           │                ╰─ sage_data [40]
          │  │        │  │           ╰─ s [51]
          │  │        │  │              ╰─ /
          │  │        │  │                 ├─ new [55]
          │  │        │  │                 ╰─ {id} [52]
          │  │        │  │                       ╰─ /
          │  │        │  │                          ├─ edit [53]
          │  │        │  │                          ╰─ renew [54]
          │  │        │  ╰─ udit_log
          │  │        │            ├─ _reports [56]
          │  │        │            ╰─ s [57]
          │  │        ├─ b
          │  │        │  ├─ ackground_
          │  │        │  │           ├─ jobs [58]
          │  │        │  │           ╰─ migrations [59]
          │  │        │  │                       ╰─ /
          │  │        │  │                          ├─ {id} [60]
          │  │        │  │                          │     ╰─ /
          │  │        │  │                          │        ├─ pause [61]
          │  │        │  │                          │        ╰─ re
          │  │        │  │                          │            ├─ sume [62]
          │  │        │  │                          │            ╰─ try [63]
          │  │        │  │                          ╰─ {background_migration_id}
          │  │        │  │                                                     ╰─ /batched_jobs/
          │  │        │  │                                                                     ╰─ {id} [64]
          │  │        │  ╰─ roadcast_messages [65]
          │  │        │                     ╰─ /
          │  │        │                        ├─ preview [68]
          │  │        │                        ╰─ {id} [66]
          │  │        │                              ╰─ /edit [67]
          │  │        ├─ c
          │  │        │  ├─ i/variables [69]
          │  │        │  ├─ lusters [70]
          │  │        │  │        ╰─ /
          │  │        │  │           ├─ c
          │  │        │  │           │  ├─ onnect [77]
          │  │        │  │           │  ╰─ reate_user [78]
          │  │        │  │           ├─ new_cluster_docs [79]
          │  │        │  │           ├─ {id} [71]
          │  │        │  │           │     ╰─ /
          │  │        │  │           │        ├─ cl
          │  │        │  │           │        │   ├─ ear_cache [72]
          │  │        │  │           │        │   ╰─ uster_status [73]
          │  │        │  │           │        ├─ environments [74]
          │  │        │  │           │        ╰─ metrics [75]
          │  │        │  │           │                 ╰─ _dashboard [76]
          │  │        │  │           ╰─ {cluster_id}
          │  │        │  │                         ╰─ /integration/create_or_update [80]
          │  │        │  ├─ o
          │  │        │  │  ├─ de_suggestions [81]
          │  │        │  │  ╰─ horts [82]
          │  │        │  ╰─ redentials [83]
          │  │        │              ╰─ /
          │  │        │                 ├─ {credential_id}
          │  │        │                 │                ╰─ /resources/
          │  │        │                 │                             ╰─ {resource_id}
          │  │        │                 │                                            ╰─ /revoke [84]
          │  │        │                 ╰─ {id} [85]
          │  │        │                       ╰─ /revoke [86]
          │  │        ├─ d
          │  │        │  ├─ ashboard/stats [88]
          │  │        │  ╰─ e
          │  │        │     ├─ ploy_keys [89]
          │  │        │     │          ╰─ /
          │  │        │     │             ├─ new [92]
          │  │        │     │             ╰─ {id} [90]
          │  │        │     │                   ╰─ /edit [91]
          │  │        │     ╰─ v_ops_reports [93]
          │  │        ├─ e
          │  │        │  ├─ lasticsearch/
          │  │        │  │              ├─ cancel_index_deletion [94]
          │  │        │  │              ├─ enqueue_index [95]
          │  │        │  │              ├─ retry_migration [96]
          │  │        │  │              ╰─ trigger_reindexing [97]
          │  │        │  ╰─ mail [98]
          │  │        ├─ g
          │  │        │  ├─ eo [99]
          │  │        │  │   ╰─ /
          │  │        │  │      ├─ s
          │  │        │  │      │  ├─ ites [100]
          │  │        │  │      │  │     ╰─ /
          │  │        │  │      │  │        ├─ new [104]
          │  │        │  │      │  │        ╰─ {id} [101]
          │  │        │  │      │  │              ╰─ /
          │  │        │  │      │  │                 ├─ edit [102]
          │  │        │  │      │  │                 ╰─ replication [103]
          │  │        │  │      │  │                              ╰─ /
          │  │        │  │      │  │                                 ╰─ {replicable_name_plural} [106]
          │  │        │  │      │  ╰─ ettings [108]
          │  │        │  │      ╰─ replication/
          │  │        │  │                    ╰─ {replicable_name_plural} [105]
          │  │        │  ├─ italy_servers [109]
          │  │        │  ╰─ roups [110]
          │  │        │         ╰─ /
          │  │        │            ├─ new [115]
          │  │        │            ├─ {id:*}
          │  │        │            │       ╰─ /
          │  │        │            │          ├─ edit [112]
          │  │        │            │          ├─ members_update [113]
          │  │        │            │          ╰─ reset_runners_minutes [114]
          │  │        │            ╰─ {id:*} [111]
          │  │        ├─ h
          │  │        │  ├─ ealth_check [116]
          │  │        │  ╰─ ooks [119]
          │  │        │        ╰─ /
          │  │        │           ├─ {hook_id}
          │  │        │           │          ╰─ /hook_logs/
          │  │        │           │                       ╰─ {id} [117]
          │  │        │           │                             ╰─ /retry [118]
          │  │        │           ╰─ {id} [120]
          │  │        │                 ╰─ /
          │  │        │                    ├─ edit [121]
          │  │        │                    ╰─ test [122]
          │  │        ├─ us
          │  │        │   ├─ er
          │  │        │   │   ├─ s [191]
          │  │        │   │   │  ╰─ /
          │  │        │   │   │     ├─ new [216]
          │  │        │   │   │     ├─ {user_id}
          │  │        │   │   │     │          ╰─ /
          │  │        │   │   │     │             ├─ i
          │  │        │   │   │     │             │  ├─ dentities [123]
          │  │        │   │   │     │             │  │          ╰─ /
          │  │        │   │   │     │             │  │             ├─ new [126]
          │  │        │   │   │     │             │  │             ╰─ {id} [124]
          │  │        │   │   │     │             │  │                   ╰─ /edit [125]
          │  │        │   │   │     │             │  ╰─ mpersonation_tokens [127]
          │  │        │   │   │     │             │                       ╰─ /
          │  │        │   │   │     │             │                          ╰─ {id}
          │  │        │   │   │     │             │                                ╰─ /revoke [128]
          │  │        │   │   │     │             ╰─ keys/
          │  │        │   │   │     │                    ╰─ {id} [140]
          │  │        │   │   │     ╰─ {id} [192]
          │  │        │   │   │           ╰─ /
          │  │        │   │   │              ├─ a
          │  │        │   │   │              │  ├─ ctivate [193]
          │  │        │   │   │              │  ╰─ pprove [194]
          │  │        │   │   │              ├─ b
          │  │        │   │   │              │  ├─ an [195]
          │  │        │   │   │              │  ╰─ lock [196]
          │  │        │   │   │              ├─ c
          │  │        │   │   │              │  ├─ ard_match [197]
          │  │        │   │   │              │  ╰─ onfirm [198]
          │  │        │   │   │              ├─ d
          │  │        │   │   │              │  ├─ e
          │  │        │   │   │              │  │  ├─ activate [199]
          │  │        │   │   │              │  │  ╰─ stroy_identity_verification_exemption [200]
          │  │        │   │   │              │  ╰─ isable_two_factor [201]
          │  │        │   │   │              ├─ edit [202]
          │  │        │   │   │              ├─ i
          │  │        │   │   │              │  ├─ dentity_verification_exemption [203]
          │  │        │   │   │              │  ╰─ mpersonate [204]
          │  │        │   │   │              ├─ keys [205]
          │  │        │   │   │              ├─ p
          │  │        │   │   │              │  ├─ hone_match [206]
          │  │        │   │   │              │  ╰─ rojects [207]
          │  │        │   │   │              ├─ re
          │  │        │   │   │              │   ├─ ject [208]
          │  │        │   │   │              │   ├─ move/
          │  │        │   │   │              │   │      ╰─ {email_id} [209]
          │  │        │   │   │              │   ╰─ set_runners_minutes [210]
          │  │        │   │   │              ├─ trust [211]
          │  │        │   │   │              ╰─ un
          │  │        │   │   │                  ├─ b
          │  │        │   │   │                  │  ├─ an [212]
          │  │        │   │   │                  │  ╰─ lock [213]
          │  │        │   │   │                  ├─ lock [214]
          │  │        │   │   │                  ╰─ trust [215]
          │  │        │   │   ╰─ _permission_exports [190]
          │  │        │   ╰─ age_trends [189]
          │  │        ├─ i
          │  │        │  ├─ mpersonation [129]
          │  │        │  ╰─ n
          │  │        │     ├─ itial_setup [130]
          │  │        │     │            ╰─ /new [131]
          │  │        │     ╰─ stance_review [132]
          │  │        ├─ jobs [138]
          │  │        │     ╰─ /cancel_all [139]
          │  │        ├─ l
          │  │        │  ├─ abels [141]
          │  │        │  │      ╰─ /
          │  │        │  │         ├─ new [144]
          │  │        │  │         ╰─ {id} [142]
          │  │        │  │               ╰─ /edit [143]
          │  │        │  ╰─ icense [145]
          │  │        │          ╰─ /
          │  │        │             ├─ download [146]
          │  │        │             ├─ sync_seat_link [147]
          │  │        │             ╰─ usage_export [148]
          │  │        ├─ namespace_limits [149]
          │  │        │                 ╰─ /export_usage [150]
          │  │        ├─ organizations [151]
          │  │        ├─ p
          │  │        │  ├─ lan_limits [152]
          │  │        │  ├─ rojects [153]
          │  │        │  │        ╰─ /
          │  │        │  │           ╰─ {namespace_id:*}
          │  │        │  │                             ╰─ /
          │  │        │  │                                ├─ {id} [154]
          │  │        │  │                                │     ╰─ /
          │  │        │  │                                │        ├─ edit [155]
          │  │        │  │                                │        ├─ repository_check [156]
          │  │        │  │                                │        ╰─ transfer [157]
          │  │        │  │                                ╰─ {project_id}
          │  │        │  │                                              ╰─ /runner_projects [160]
          │  │        │  │                                                                ╰─ /
          │  │        │  │                                                                   ╰─ {id} [161]
          │  │        │  ╰─ ush_rule [158]
          │  │        ├─ r
          │  │        │  ├─ ole_promotion_requests [159]
          │  │        │  ╰─ unners [162]
          │  │        │          ╰─ /
          │  │        │             ├─ dashboard [168]
          │  │        │             ├─ new [169]
          │  │        │             ├─ runner_setup_scripts [170]
          │  │        │             ├─ tag_list [171]
          │  │        │             ╰─ {id} [163]
          │  │        │                   ╰─ /
          │  │        │                      ├─ edit [164]
          │  │        │                      ├─ pause [165]
          │  │        │                      ╰─ re
          │  │        │                          ├─ gister [166]
          │  │        │                          ╰─ sume [167]
          │  │        ├─ s
          │  │        │  ├─ ession [172]
          │  │        │  │       ╰─ /
          │  │        │  │          ├─ destroy [173]
          │  │        │  │          ╰─ new [174]
          │  │        │  ├─ pam_logs [177]
          │  │        │  │         ╰─ /
          │  │        │  │            ╰─ {id} [178]
          │  │        │  │                  ╰─ /mark_as_ham [179]
          │  │        │  ├─ ubscription [180]
          │  │        │  ╰─ ystem_info [181]
          │  │        ├─ topics [182]
          │  │        │       ╰─ /
          │  │        │          ├─ merge [185]
          │  │        │          ├─ new [186]
          │  │        │          ├─ preview_markdown [187]
          │  │        │          ├─ {id} [183]
          │  │        │          │     ╰─ /edit [184]
          │  │        │          ╰─ {topic_id}
          │  │        │                      ╰─ /avatar [188]
          │  │        ╰─ version_check [217]
          │  ╰─ pi/
          │       ├─ graphql [285]
          │       ╰─ v4/geo/graphql [286]
          ├─ u
          │  ├─ sers [1418]
          │  │     ╰─ /
          │  │        ├─ a
          │  │        │  ├─ lmost_there [245]
          │  │        │  ╰─ uth/
          │  │        │        ├─ -/import/google_oauth2/callback [659]
          │  │        │        ├─ g
          │  │        │        │  ├─ oogle_oauth2 [714]
          │  │        │        │  │             ╰─ /callback [715]
          │  │        │        │  ╰─ eo/sign_
          │  │        │        │            ├─ in [1488]
          │  │        │        │            ╰─ out [1489]
          │  │        │        ╰─ kerberos/negotiate [716]
          │  │        ├─ c
          │  │        │  ├─ onfirmation [246]
          │  │        │  │            ╰─ /new [247]
          │  │        │  ╰─ ancel [1419]
          │  │        ├─ u
          │  │        │  ├─ nlock [267]
          │  │        │  │      ╰─ /new [268]
          │  │        │  ╰─ pdate_email [1494]
          │  │        ├─ password [731]
          │  │        │         ╰─ /
          │  │        │            ├─ edit [732]
          │  │        │            ╰─ new [733]
          │  │        ├─ edit [1420]
          │  │        ├─ s
          │  │        │  ├─ ign_
          │  │        │  │     ├─ up [1421]
          │  │        │  │     │   ╰─ /
          │  │        │  │     │      ├─ company [1422]
          │  │        │  │     │      │        ╰─ /new [1423]
          │  │        │  │     │      ├─ groups [1424]
          │  │        │  │     │      │       ╰─ /new [1425]
          │  │        │  │     │      ╰─ welcome [1426]
          │  │        │  │     ├─ in [1491]
          │  │        │  │     ╰─ out [1492]
          │  │        │  ╰─ uccessful_verification [1493]
          │  │        ├─ resend_verification_code [1490]
          │  │        ├─ identity_verification [1592]
          │  │        │                      ╰─ /
          │  │        │                         ├─ arkose_labs_challenge [1593]
          │  │        │                         ├─ res
          │  │        │                         │    ├─ end_email_code [1594]
          │  │        │                         │    ╰─ tricted [1595]
          │  │        │                         ├─ s
          │  │        │                         │  ├─ end_phone_verification_code [1596]
          │  │        │                         │  ╰─ uccess [1597]
          │  │        │                         ├─ toggle_phone_exemption [1598]
          │  │        │                         ╰─ verif
          │  │        │                                ├─ ication_state [1599]
          │  │        │                                ╰─ y_
          │  │        │                                    ├─ arkose_labs_session [1600]
          │  │        │                                    ├─ credit_card [1601]
          │  │        │                                    │            ╰─ _captcha [1602]
          │  │        │                                    ├─ email_code [1603]
          │  │        │                                    ╰─ phone_verification_code [1604]
          │  │        ╰─ {username}
          │  │                    ╰─ /
          │  │                       ├─ a
          │  │                       │  ├─ ctivity [1563]
          │  │                       │  ╰─ vailable_
          │  │                       │             ├─ group_templates [1564]
          │  │                       │             ╰─ project_templates [1565]
          │  │                       ├─ c
          │  │                       │  ├─ alendar [1566]
          │  │                       │  │        ╰─ _activities [1567]
          │  │                       │  ╰─ ontributed [1568]
          │  │                       ├─ exists [1569]
          │  │                       ├─ follow [1570]
          │  │                       │       ├─ ers [1571]
          │  │                       │       ╰─ ing [1572]
          │  │                       ├─ groups [1573]
          │  │                       ├─ projects [1574]
          │  │                       ├─ s
          │  │                       │  ├─ nippets [1575]
          │  │                       │  ╰─ tarred [1576]
          │  │                       ╰─ unfollow [1577]
          │  ├─ ploads/
          │  │        ├─ -/system/
          │  │        │          ├─ temp/
          │  │        │          │      ╰─ {secret}
          │  │        │          │                ╰─ /
          │  │        │          │                   ╰─ {filename} [1535]
          │  │        │          ╰─ {model}
          │  │        │                   ╰─ /
          │  │        │                      ├─ {id}
          │  │        │                      │     ╰─ /
          │  │        │                      │        ╰─ {secret}
          │  │        │                      │                  ╰─ /
          │  │        │                      │                     ╰─ {filename} [1533]
          │  │        │                      ╰─ {mounted_as}
          │  │        │                                    ╰─ /
          │  │        │                                       ╰─ {id}
          │  │        │                                             ╰─ /
          │  │        │                                                ╰─ {filename} [1534]
          │  │        ╰─ {model} [1536]
          │  │                 ╰─ /authorize [1537]
          │  ╰─ nsubscribes/
          │                ╰─ {email} [1608]
          ├─ dashboard [257]
          │          ╰─ /
          │             ├─ activity [251]
          │             ├─ issues [252]
          │             ├─ m
          │             │  ├─ erge_requests [253]
          │             │  ╰─ ilestones [256]
          │             ├─ groups [254]
          │             ├─ labels [255]
          │             ├─ projects [258]
          │             │         ╰─ /
          │             │            ├─ removed [259]
          │             │            ╰─ starred [260]
          │             ├─ snippets [261]
          │             ╰─ todos [262]
          │                    ╰─ /
          │                       ├─ bulk_restore [265]
          │                       ├─ destroy_all [266]
          │                       ╰─ {id} [263]
          │                             ╰─ /restore [264]
          ├─ oauth/
          │       ├─ userinfo [269]
          │       ├─ discovery/keys [691]
          │       ├─ a
          │       │  ├─ pplications [698]
          │       │  │            ╰─ /
          │       │  │               ├─ new [702]
          │       │  │               ╰─ {id} [699]
          │       │  │                     ╰─ /
          │       │  │                        ├─ edit [700]
          │       │  │                        ╰─ renew [701]
          │       │  ╰─ uthorize [703]
          │       │            ├─ /native [704]
          │       │            ╰─ d_applications [705]
          │       │                            ╰─ /
          │       │                               ╰─ {id} [706]
          │       ├─ geo/
          │       │     ├─ auth [707]
          │       │     ├─ callback [708]
          │       │     ╰─ logout [709]
          │       ├─ token [713]
          │       │      ╰─ /info [710]
          │       ├─ introspect [711]
          │       ╰─ revoke [712]
          ├─ explore [274]
          │        ╰─ /
          │           ├─ catalog [270]
          │           │        ╰─ /
          │           │           ╰─ {full_path:*} [271]
          │           ├─ dependencies [272]
          │           ├─ groups [273]
          │           ├─ projects [275]
          │           │         ╰─ /
          │           │            ├─ starred [276]
          │           │            ╰─ t
          │           │               ├─ opics [277]
          │           │               │      ╰─ /
          │           │               │         ╰─ {topic_name} [278]
          │           │               ╰─ rending [279]
          │           ╰─ snippets [282]
          ├─ p
          │  ├─ ublic [280]
          │  │      ╰─ /projects [281]
          │  ╰─ rojects [783]
          │           ╰─ /
          │              ├─ new [784]
          │              ╰─ {id} [1262]
          ├─ groups [288]
          │       ╰─ /
          │          ├─ new [305]
          │          ├─ {group_id}
          │          │           ╰─ /preview_markdown [304]
          │          ├─ {group_id:*}
          │          │             ╰─ /-/
          │          │                  ├─ p
          │          │                  │  ├─ r
          │          │                  │  │  ├─ eview_markdown [289]
          │          │                  │  │  ╰─ otected_
          │          │                  │  │            ├─ branches [478]
          │          │                  │  │            │         ╰─ /
          │          │                  │  │            │            ╰─ {id} [479]
          │          │                  │  │            ╰─ environments [480]
          │          │                  │  │                          ╰─ /
          │          │                  │  │                             ╰─ {id} [481]
          │          │                  │  ├─ ackages [476]
          │          │                  │  │        ╰─ /
          │          │                  │  │           ╰─ {id} [477]
          │          │                  │  ╰─ ush_rules [482]
          │          │                  ├─ r
          │          │                  │  ├─ e
          │          │                  │  │  ├─ store [290]
          │          │                  │  │  ╰─ leases [485]
          │          │                  │  ├─ oadmap [486]
          │          │                  │  ╰─ unners [487]
          │          │                  │          ╰─ /
          │          │                  │             ├─ dashboard [493]
          │          │                  │             ├─ new [494]
          │          │                  │             ╰─ {id} [488]
          │          │                  │                   ╰─ /
          │          │                  │                      ├─ edit [489]
          │          │                  │                      ├─ pause [490]
          │          │                  │                      ╰─ re
          │          │                  │                          ├─ gister [491]
          │          │                  │                          ╰─ sume [492]
          │          │                  ├─ a
          │          │                  │  ├─ chievements [306]
          │          │                  │  │            ╰─ /
          │          │                  │  │               ├─ new [308]
          │          │                  │  │               ╰─ {id}
          │          │                  │  │                     ╰─ /edit [307]
          │          │                  │  ├─ dd_ons/discover_duo_pro [309]
          │          │                  │  ├─ nalytics/
          │          │                  │  │          ├─ c
          │          │                  │  │          │  ├─ i_cd [310]
          │          │                  │  │          │  ╰─ overage_reports [311]
          │          │                  │  │          ├─ value_stream_analytics [312]
          │          │                  │  │          │                       ╰─ /
          │          │                  │  │          │                          ├─ value_streams [323]
          │          │                  │  │          │                          │              ╰─ /
          │          │                  │  │          │                          │                 ├─ new [326]
          │          │                  │  │          │                          │                 ├─ {value_stream_id}
          │          │                  │  │          │                          │                 │                  ╰─ /stages [313]
          │          │                  │  │          │                          │                 │                           ╰─ /
          │          │                  │  │          │                          │                 │                              ╰─ {id}
          │          │                  │  │          │                          │                 │                                    ╰─ /
          │          │                  │  │          │                          │                 │                                       ├─ average [314]
          │          │                  │  │          │                          │                 │                                       │        ╰─ _duration_chart [315]
          │          │                  │  │          │                          │                 │                                       ├─ count [316]
          │          │                  │  │          │                          │                 │                                       ├─ median [317]
          │          │                  │  │          │                          │                 │                                       ╰─ records [318]
          │          │                  │  │          │                          │                 ╰─ {id} [324]
          │          │                  │  │          │                          │                       ╰─ /edit [325]
          │          │                  │  │          │                          ├─ cycle_times [319]
          │          │                  │  │          │                          ├─ lead_times [320]
          │          │                  │  │          │                          ├─ summary [321]
          │          │                  │  │          │                          ╰─ time_summary [322]
          │          │                  │  │          ├─ d
          │          │                  │  │          │  ├─ ashboards [327]
          │          │                  │  │          │  │          ╰─ /
          │          │                  │  │          │  │             ╰─ {vueroute:*} [328]
          │          │                  │  │          │  ╰─ evops_adoption [329]
          │          │                  │  │          ├─ merge_request_analytics [330]
          │          │                  │  │          ├─ productivity_analytics [331]
          │          │                  │  │          ├─ repository_analytics [332]
          │          │                  │  │          ╰─ type_of_work/tasks_by_type [333]
          │          │                  │  │                                      ╰─ /top_labels [334]
          │          │                  │  ├─ u
          │          │                  │  │  ├─ dit_events [335]
          │          │                  │  │  ╰─ tocomplete_sources/
          │          │                  │  │                       ├─ commands [336]
          │          │                  │  │                       ├─ epics [337]
          │          │                  │  │                       ├─ i
          │          │                  │  │                       │  ├─ ssues [338]
          │          │                  │  │                       │  ╰─ terations [339]
          │          │                  │  │                       ├─ labels [340]
          │          │                  │  │                       ├─ m
          │          │                  │  │                       │  ├─ e
          │          │                  │  │                       │  │  ├─ mbers [341]
          │          │                  │  │                       │  │  ╰─ rge_requests [342]
          │          │                  │  │                       │  ╰─ ilestones [343]
          │          │                  │  │                       ├─ vulnerabilities [344]
          │          │                  │  │                       ╰─ wikis [345]
          │          │                  │  ╰─ vatar [346]
          │          │                  ├─ b
          │          │                  │  ├─ illings [347]
          │          │                  │  │        ╰─ /refresh_seats [348]
          │          │                  │  ╰─ oards [349]
          │          │                  │         ╰─ /
          │          │                  │            ╰─ {id} [350]
          │          │                  ├─ c
          │          │                  │  ├─ hildren [351]
          │          │                  │  ├─ lusters [352]
          │          │                  │  │        ╰─ /
          │          │                  │  │           ├─ c
          │          │                  │  │           │  ├─ onnect [359]
          │          │                  │  │           │  ╰─ reate_user [360]
          │          │                  │  │           ├─ new_cluster_docs [361]
          │          │                  │  │           ├─ {id} [353]
          │          │                  │  │           │     ╰─ /
          │          │                  │  │           │        ├─ cl
          │          │                  │  │           │        │   ├─ ear_cache [354]
          │          │                  │  │           │        │   ╰─ uster_status [355]
          │          │                  │  │           │        ├─ environments [356]
          │          │                  │  │           │        ╰─ metrics [357]
          │          │                  │  │           │                 ╰─ _dashboard [358]
          │          │                  │  │           ╰─ {cluster_id}
          │          │                  │  │                         ╰─ /integration/create_or_update [362]
          │          │                  │  ├─ o
          │          │                  │  │  ├─ mment_templates [363]
          │          │                  │  │  │                ╰─ /
          │          │                  │  │  │                   ╰─ {id} [364]
          │          │                  │  │  ╰─ nt
          │          │                  │  │      ├─ ribution_analytics [366]
          │          │                  │  │      ╰─ ainer_registries [483]
          │          │                  │  │                        ╰─ /
          │          │                  │  │                           ╰─ {id} [484]
          │          │                  │  ├─ rm/
          │          │                  │  │    ├─ contacts [367]
          │          │                  │  │    │         ╰─ /
          │          │                  │  │    │            ├─ new [369]
          │          │                  │  │    │            ╰─ {id}
          │          │                  │  │    │                  ╰─ /edit [368]
          │          │                  │  │    ╰─ organizations [370]
          │          │                  │  │                   ╰─ /
          │          │                  │  │                      ├─ new [372]
          │          │                  │  │                      ╰─ {id}
          │          │                  │  │                            ╰─ /edit [371]
          │          │                  │  ├─ ustom_emoji [373]
          │          │                  │  │            ╰─ /new [374]
          │          │                  │  ╰─ adences [437]
          │          │                  │           ╰─ /
          │          │                  │              ├─ new [452]
          │          │                  │              ├─ {id} [446]
          │          │                  │              │     ╰─ /edit [447]
          │          │                  │              ├─ {iteration_cadence_id}
          │          │                  │              │                       ╰─ /iterations [448]
          │          │                  │              │                                    ╰─ /
          │          │                  │              │                                       ├─ new [451]
          │          │                  │              │                                       ╰─ {id} [449]
          │          │                  │              │                                             ╰─ /edit [450]
          │          │                  │              ├─ {vueroute:*}
          │          │                  │              │             ╰─ /
          │          │                  │              │                ├─ new [445]
          │          │                  │              │                ├─ {id} [439]
          │          │                  │              │                │     ╰─ /edit [440]
          │          │                  │              │                ╰─ {iteration_cadence_id}
          │          │                  │              │                                        ╰─ /iterations [441]
          │          │                  │              │                                                     ╰─ /
          │          │                  │              │                                                        ├─ new [444]
          │          │                  │              │                                                        ╰─ {id} [442]
          │          │                  │              │                                                              ╰─ /edit [443]
          │          │                  │              ╰─ {vueroute:*} [438]
          │          │                  ├─ w
          │          │                  │  ├─ ikis [572]
          │          │                  │  │     ╰─ /
          │          │                  │  │        ├─ -/confluence [365]
          │          │                  │  │        ├─ git_access [579]
          │          │                  │  │        ├─ new [580]
          │          │                  │  │        ├─ pages [581]
          │          │                  │  │        ├─ templates [582]
          │          │                  │  │        ├─ {id:*}
          │          │                  │  │        │       ╰─ /
          │          │                  │  │        │          ├─ diff [574]
          │          │                  │  │        │          ├─ edit [575]
          │          │                  │  │        │          ├─ history [576]
          │          │                  │  │        │          ├─ preview_markdown [577]
          │          │                  │  │        │          ╰─ raw [578]
          │          │                  │  │        ╰─ {id:*} [573]
          │          │                  │  ╰─ ork_items [583]
          │          │                  │             ╰─ /
          │          │                  │                ╰─ {iid} [584]
          │          │                  │                       ╰─ /descriptions/
          │          │                  │                                       ╰─ {version_id} [585]
          │          │                  │                                                     ╰─ /diff [586]
          │          │                  ├─ d
          │          │                  │  ├─ ep
          │          │                  │  │   ├─ endenc
          │          │                  │  │   │       ├─ ies [375]
          │          │                  │  │   │       │    ╰─ /l
          │          │                  │  │   │       │        ├─ icenses [376]
          │          │                  │  │   │       │        ╰─ ocations [377]
          │          │                  │  │   │       ╰─ y_proxy [378]
          │          │                  │  │   ╰─ loy_tokens/
          │          │                  │  │                ╰─ {id}
          │          │                  │  │                      ╰─ /revoke [386]
          │          │                  │  ╰─ iscover [387]
          │          │                  ├─ epic
          │          │                  │     ├─ _boards [388]
          │          │                  │     │        ╰─ /
          │          │                  │     │           ╰─ {id} [389]
          │          │                  │     ╰─ s [392]
          │          │                  │        ╰─ /
          │          │                  │           ├─ bulk_update [401]
          │          │                  │           ├─ new [402]
          │          │                  │           ├─ {epic_id}
          │          │                  │           │          ╰─ /
          │          │                  │           │             ├─ issues [390]
          │          │                  │           │             │       ╰─ /
          │          │                  │           │             │          ╰─ {id} [391]
          │          │                  │           │             ├─ links [403]
          │          │                  │           │             │      ╰─ /
          │          │                  │           │             │         ╰─ {id} [404]
          │          │                  │           │             ├─ notes [405]
          │          │                  │           │             │      ╰─ /
          │          │                  │           │             │         ╰─ {id} [406]
          │          │                  │           │             │               ╰─ /toggle_award_emoji [407]
          │          │                  │           │             ╰─ related_epic_links [408]
          │          │                  │           │                                 ╰─ /
          │          │                  │           │                                    ╰─ {id} [409]
          │          │                  │           ╰─ {id} [393]
          │          │                  │                 ╰─ /
          │          │                  │                    ├─ d
          │          │                  │                    │  ├─ escriptions/
          │          │                  │                    │  │             ╰─ {version_id} [394]
          │          │                  │                    │  │                           ╰─ /diff [395]
          │          │                  │                    │  ╰─ iscussions [396]
          │          │                  │                    ├─ edit [397]
          │          │                  │                    ├─ realtime_changes [398]
          │          │                  │                    ╰─ toggle_
          │          │                  │                             ├─ award_emoji [399]
          │          │                  │                             ╰─ subscription [400]
          │          │                  ├─ group_
          │          │                  │       ├─ links/
          │          │                  │       │       ╰─ {id} [410]
          │          │                  │       ╰─ members [411]
          │          │                  │                ╰─ /
          │          │                  │                   ├─ export_csv [418]
          │          │                  │                   ├─ leave [419]
          │          │                  │                   ├─ request_access [420]
          │          │                  │                   ╰─ {id} [412]
          │          │                  │                         ╰─ /
          │          │                  │                            ├─ approve_access_request [413]
          │          │                  │                            ├─ ban [414]
          │          │                  │                            ├─ override [415]
          │          │                  │                            ├─ resend_invite [416]
          │          │                  │                            ╰─ unban [417]
          │          │                  ├─ h
          │          │                  │  ├─ arbor/repositories [422]
          │          │                  │  │                   ╰─ /
          │          │                  │  │                      ├─ {repository_id}
          │          │                  │  │                      │                ╰─ /artifacts [421]
          │          │                  │  │                      │                            ╰─ /
          │          │                  │  │                      │                               ╰─ {artifact_id}
          │          │                  │  │                      │                                              ╰─ /tags [424]
          │          │                  │  │                      ╰─ {id} [423]
          │          │                  │  ╰─ ooks [427]
          │          │                  │        ╰─ /
          │          │                  │           ├─ {hook_id}
          │          │                  │           │          ╰─ /hook_logs/
          │          │                  │           │                       ╰─ {id} [425]
          │          │                  │           │                             ╰─ /retry [426]
          │          │                  │           ╰─ {id} [428]
          │          │                  │                 ╰─ /
          │          │                  │                    ├─ edit [429]
          │          │                  │                    ╰─ test [430]
          │          │                  ├─ i
          │          │                  │  ├─ mport [431]
          │          │                  │  ├─ nsights [433]
          │          │                  │  │        ╰─ /query [434]
          │          │                  │  ├─ ssues
          │          │                  │  │      ├─ /bulk_update [435]
          │          │                  │  │      ╰─ _analytics [436]
          │          │                  │  ╰─ terations [453]
          │          │                  │             ╰─ /
          │          │                  │                ├─ new [456]
          │          │                  │                ╰─ {id} [454]
          │          │                  │                      ╰─ /edit [455]
          │          │                  ├─ t
          │          │                  │  ├─ erraform_module_registry [432]
          │          │                  │  ├─ odos [564]
          │          │                  │  ╰─ wo_factor_auth [565]
          │          │                  ├─ l
          │          │                  │  ├─ abels [457]
          │          │                  │  │      ╰─ /
          │          │                  │  │         ├─ new [461]
          │          │                  │  │         ╰─ {id} [458]
          │          │                  │  │               ╰─ /
          │          │                  │  │                  ├─ edit [459]
          │          │                  │  │                  ╰─ toggle_subscription [460]
          │          │                  │  ╰─ dap
          │          │                  │       ├─ _group_links [462]
          │          │                  │       │             ╰─ /
          │          │                  │       │                ╰─ {id} [463]
          │          │                  │       ╰─ /sync [464]
          │          │                  ├─ m
          │          │                  │  ├─ erge_requests/bulk_update [465]
          │          │                  │  ╰─ ilestones [466]
          │          │                  │             ╰─ /
          │          │                  │                ├─ new [473]
          │          │                  │                ╰─ {id} [467]
          │          │                  │                      ╰─ /
          │          │                  │                         ├─ edit [468]
          │          │                  │                         ├─ issues [469]
          │          │                  │                         ├─ labels [470]
          │          │                  │                         ├─ merge_requests [471]
          │          │                  │                         ╰─ participants [472]
          │          │                  ├─ notification_setting [474]
          │          │                  ├─ s
          │          │                  │  ├─ aml [497]
          │          │                  │  │    ├─ /
          │          │                  │  │    │  ├─ callback [475]
          │          │                  │  │    │  ├─ u
          │          │                  │  │    │  │  ├─ pdate_microsoft_application [498]
          │          │                  │  │    │  │  ╰─ nlink [563]
          │          │                  │  │    │  ╰─ sso [562]
          │          │                  │  │    ╰─ _group_links [495]
          │          │                  │  │                  ╰─ /
          │          │                  │  │                     ╰─ {id} [496]
          │          │                  │  ├─ cim_oauth [499]
          │          │                  │  ├─ e
          │          │                  │  │  ├─ at_usage [500]
          │          │                  │  │  ├─ curity/
          │          │                  │  │  │        ├─ c
          │          │                  │  │  │        │  ├─ ompliance_
          │          │                  │  │  │        │  │           ├─ dashboard [501]
          │          │                  │  │  │        │  │           │          ╰─ /
          │          │                  │  │  │        │  │           │             ╰─ {vueroute:*} [502]
          │          │                  │  │  │        │  │           ├─ framework_reports [503]
          │          │                  │  │  │        │  │           ├─ project_framework_reports [504]
          │          │                  │  │  │        │  │           ├─ standards_adherence_reports [505]
          │          │                  │  │  │        │  │           ╰─ violation_reports [506]
          │          │                  │  │  │        │  ╰─ redentials [507]
          │          │                  │  │  │        │              ╰─ /
          │          │                  │  │  │        │                 ╰─ {id} [508]
          │          │                  │  │  │        │                       ╰─ /revoke [509]
          │          │                  │  │  │        ├─ d
          │          │                  │  │  │        │  ├─ ashboard [510]
          │          │                  │  │  │        │  ╰─ iscover [511]
          │          │                  │  │  │        ├─ merge_commit_reports [512]
          │          │                  │  │  │        ├─ policies [513]
          │          │                  │  │  │        │         ╰─ /
          │          │                  │  │  │        │            ├─ new [515]
          │          │                  │  │  │        │            ├─ schema [516]
          │          │                  │  │  │        │            ╰─ {id}
          │          │                  │  │  │        │                  ╰─ /edit [514]
          │          │                  │  │  │        ╰─ vulnerabilities [517]
          │          │                  │  │  ├─ rvice_accounts [518]
          │          │                  │  │  │               ╰─ /
          │          │                  │  │  │                  ├─ new [525]
          │          │                  │  │  │                  ├─ {id} [523]
          │          │                  │  │  │                  │     ╰─ /edit [524]
          │          │                  │  │  │                  ├─ {vueroute:*}
          │          │                  │  │  │                  │             ╰─ /
          │          │                  │  │  │                  │                ├─ new [522]
          │          │                  │  │  │                  │                ╰─ {id} [520]
          │          │                  │  │  │                  │                      ╰─ /edit [521]
          │          │                  │  │  │                  ╰─ {vueroute:*} [519]
          │          │                  │  │  ╰─ ttings/
          │          │                  │  │           ├─ a
          │          │                  │  │           │  ├─ ccess_tokens [526]
          │          │                  │  │           │  │             ╰─ /
          │          │                  │  │           │  │                ╰─ {id}
          │          │                  │  │           │  │                      ╰─ /revoke [527]
          │          │                  │  │           │  ├─ nalytics [528]
          │          │                  │  │           │  ╰─ pplications [529]
          │          │                  │  │           │               ╰─ /
          │          │                  │  │           │                  ├─ new [533]
          │          │                  │  │           │                  ╰─ {id} [530]
          │          │                  │  │           │                        ╰─ /
          │          │                  │  │           │                           ├─ edit [531]
          │          │                  │  │           │                           ╰─ renew [532]
          │          │                  │  │           ├─ ci_cd [534]
          │          │                  │  │           │      ╰─ /
          │          │                  │  │           │         ├─ r
          │          │                  │  │           │         │  ├─ eset_registration_token [535]
          │          │                  │  │           │         │  ╰─ unner_setup_scripts [536]
          │          │                  │  │           │         ├─ update_auto_devops [537]
          │          │                  │  │           │         ╰─ deploy_token/create [553]
          │          │                  │  │           ├─ domain_verification [538]
          │          │                  │  │           │                    ╰─ /
          │          │                  │  │           │                       ├─ new [543]
          │          │                  │  │           │                       ╰─ {id} [539]
          │          │                  │  │           │                             ╰─ /
          │          │                  │  │           │                                ├─ clean_certificate [540]
          │          │                  │  │           │                                ├─ retry_auto_ssl [541]
          │          │                  │  │           │                                ╰─ verify [542]
          │          │                  │  │           ├─ integrations [544]
          │          │                  │  │           │             ╰─ /
          │          │                  │  │           │                ╰─ {id} [545]
          │          │                  │  │           │                      ╰─ /
          │          │                  │  │           │                         ├─ edit [546]
          │          │                  │  │           │                         ├─ reset [547]
          │          │                  │  │           │                         ╰─ test [548]
          │          │                  │  │           ├─ merge_requests [549]
          │          │                  │  │           ├─ packages_and_registries [550]
          │          │                  │  │           ├─ workspaces [551]
          │          │                  │  │           ├─ r
          │          │                  │  │           │  ├─ epo
          │          │                  │  │           │  │    ├─ rting [552]
          │          │                  │  │           │  │    ╰─ sitory [554]
          │          │                  │  │           │  │            ╰─ /deploy_token/create [555]
          │          │                  │  │           │  ╰─ oles_and_permissions [556]
          │          │                  │  │           │                        ╰─ /
          │          │                  │  │           │                           ├─ new [558]
          │          │                  │  │           │                           ╰─ {id}
          │          │                  │  │           │                                 ╰─ /edit [557]
          │          │                  │  │           ╰─ slack [559]
          │          │                  │  │                  ╰─ /slack_auth [560]
          │          │                  │  ╰─ hared_projects [561]
          │          │                  ├─ u
          │          │                  │  ├─ ploads [566]
          │          │                  │  │       ╰─ /
          │          │                  │  │          ├─ authorize [568]
          │          │                  │  │          ╰─ {secret}
          │          │                  │  │                    ╰─ /
          │          │                  │  │                       ╰─ {filename} [567]
          │          │                  │  ╰─ sage_quotas [569]
          │          │                  │               ╰─ /pending_members [570]
          │          │                  ╰─ variables [571]
          │          ├─ {id:*}
          │          │       ╰─ /-/
          │          │            ├─ activity [292]
          │          │            ├─ d
          │          │            │  ├─ etails [293]
          │          │            │  ╰─ ownload_export [294]
          │          │            ├─ e
          │          │            │  ├─ dit [295]
          │          │            │  ╰─ xport [296]
          │          │            ├─ i
          │          │            │  ├─ nactive [297]
          │          │            │  ╰─ ssues [298]
          │          │            ├─ merge_requests [299]
          │          │            ├─ projects [300]
          │          │            ├─ shared [301]
          │          │            ├─ transfer [302]
          │          │            ╰─ unfoldered_environment_names [303]
          │          ╰─ {id:*} [291]
          ├─ v2 [379]
          │   ╰─ /
          │      ╰─ {group_id:*}
          │                    ╰─ /dependency_proxy/containers/
          │                                                   ╰─ {image:*}
          │                                                              ╰─ /
          │                                                                 ├─ blobs/
          │                                                                 │       ╰─ {sha} [380]
          │                                                                 │              ╰─ /upload [381]
          │                                                                 │                       ╰─ /authorize [382]
          │                                                                 ╰─ manifests/
          │                                                                             ├─ {tag:*}
          │                                                                             │        ╰─ /upload [384]
          │                                                                             │                 ╰─ /authorize [385]
          │                                                                             ╰─ {tag:*} [383]
          ├─ he
          │   ├─ alth_check [589]
          │   │           ╰─ /
          │   │              ╰─ {checks} [590]
          │   ╰─ lp [591]
          │       ╰─ /
          │          ├─ d
          │          │  ├─ ocs [593]
          │          │  ╰─ rawers/
          │          │           ╰─ {markdown_file:*} [594]
          │          ├─ instance_configuration [595]
          │          ├─ shortcuts [596]
          │          ╰─ {path:*} [592]
          ├─ import/
          │        ├─ b
          │        │  ├─ itbucket [614]
          │        │  │         ├─ /
          │        │  │         │  ├─ callback [615]
          │        │  │         │  ├─ realtime_changes [616]
          │        │  │         │  ╰─ status [617]
          │        │  │         ╰─ _server [618]
          │        │  │                  ╰─ /
          │        │  │                     ├─ c
          │        │  │                     │  ├─ allback [619]
          │        │  │                     │  ╰─ onfigure [620]
          │        │  │                     ├─ new [621]
          │        │  │                     ├─ realtime_changes [622]
          │        │  │                     ╰─ status [623]
          │        │  ╰─ ulk_imports [624]
          │        │               ╰─ /
          │        │                  ├─ configure [627]
          │        │                  ├─ history [628]
          │        │                  ├─ realtime_changes [629]
          │        │                  ├─ status [630]
          │        │                  ╰─ {id}
          │        │                        ╰─ /history [625]
          │        │                                  ╰─ /
          │        │                                     ╰─ {entity_id}
          │        │                                                  ╰─ /failures [626]
          │        ├─ fogbugz [631]
          │        │        ╰─ /
          │        │           ├─ callback [632]
          │        │           ├─ new [633]
          │        │           ├─ realtime_changes [634]
          │        │           ├─ status [635]
          │        │           ╰─ user_map [636]
          │        ├─ git
          │        │    ├─ ea [637]
          │        │    │   ╰─ /
          │        │    │      ├─ new [638]
          │        │    │      ├─ personal_access_token [639]
          │        │    │      ├─ realtime_changes [640]
          │        │    │      ╰─ status [641]
          │        │    ├─ hub [642]
          │        │    │    ├─ /
          │        │    │    │  ├─ c
          │        │    │    │  │  ├─ a
          │        │    │    │  │  │  ├─ llback [643]
          │        │    │    │  │  │  ╰─ ncel [644]
          │        │    │    │  │  │        ╰─ _all [645]
          │        │    │    │  │  ╰─ ounts [646]
          │        │    │    │  ├─ details [647]
          │        │    │    │  ├─ failures [648]
          │        │    │    │  ├─ new [649]
          │        │    │    │  ├─ personal_access_token [650]
          │        │    │    │  ├─ realtime_changes [651]
          │        │    │    │  ╰─ status [652]
          │        │    │    ╰─ _group/status [653]
          │        │    ╰─ lab_
          │        │          ├─ group [654]
          │        │          │      ╰─ /authorize [655]
          │        │          ╰─ project [656]
          │        │                   ╰─ /
          │        │                      ├─ authorize [657]
          │        │                      ╰─ new [658]
          │        ├─ history [660]
          │        ├─ manifest [661]
          │        │         ╰─ /
          │        │            ├─ new [662]
          │        │            ├─ realtime_changes [663]
          │        │            ├─ status [664]
          │        │            ╰─ upload [665]
          │        ├─ source_users/
          │        │              ╰─ {id} [666]
          │        │                    ╰─ /
          │        │                       ├─ accept [667]
          │        │                       ╰─ decline [668]
          │        ╰─ url/validate [669]
          ├─ .well-known/
          │             ├─ o
          │             │  ├─ auth-authorization-server [688]
          │             │  ╰─ penid-configuration [689]
          │             ├─ webfinger [690]
          │             ├─ terraform.json [1529]
          │             ╰─ security.txt [1611]
          ├─ jwt/auth [692]
          ├─ rails/
          │       ├─ info [1413]
          │       │     ╰─ /
          │       │        ├─ properties [1414]
          │       │        ╰─ routes [1415]
          │       ╰─ mailers [1416]
          │                ╰─ /
          │                   ╰─ {path} [1417]
          ├─ s
          │  ├─ earch [1476]
          │  │      ╰─ /
          │  │         ├─ a
          │  │         │  ├─ ggregations [1477]
          │  │         │  ╰─ utocomplete [1478]
          │  │         ├─ count [1479]
          │  │         ├─ opensearch [1480]
          │  │         ╰─ settings [1481]
          │  ├─ itemap [1495]
          │  ╰─ nippets/
          │            ╰─ {id}
          │                  ╰─ /raw [1507]
          ├─ {username} [1560]
          │           ╰─ .
          │              ├─ gpg [1561]
          │              ╰─ keys [1562]
          ├─ {namespace_id:*}
          │                 ╰─ /
          │                    ├─ {project_id} [218]
          │                    │             ╰─ /
          │                    │                ├─ -/
          │                    │                │   ├─ r
          │                    │                │   │  ├─ e
          │                    │                │   │  │  ├─ leases [1271]
          │                    │                │   │  │  │       ╰─ /
          │                    │                │   │  │  │          ├─ inbox [4]
          │                    │                │   │  │  │          ├─ outbox [5]
          │                    │                │   │  │  │          ├─ new [1275]
          │                    │                │   │  │  │          ├─ permalink/latest [1276]
          │                    │                │   │  │  │          │                 ╰─ /
          │                    │                │   │  │  │          │                    ╰─ {suffix_path:*} [1277]
          │                    │                │   │  │  │          ╰─ {tag} [1272]
          │                    │                │   │  │  │                 ╰─ /
          │                    │                │   │  │  │                    ├─ downloads/
          │                    │                │   │  │  │                    │           ╰─ {filepath:*} [1273]
          │                    │                │   │  │  │                    ╰─ e
          │                    │                │   │  │  │                       ├─ dit [1274]
          │                    │                │   │  │  │                       ╰─ vidences/
          │                    │                │   │  │  │                                  ╰─ {id} [1278]
          │                    │                │   │  │  ├─ fs/
          │                    │                │   │  │  │    ├─ switch [1265]
          │                    │                │   │  │  │    ╰─ {id}
          │                    │                │   │  │  │          ╰─ /logs_tree [1263]
          │                    │                │   │  │  │                      ╰─ /
          │                    │                │   │  │  │                         ╰─ {path:*} [1264]
          │                    │                │   │  │  ├─ pository [1280]
          │                    │                │   │  │  ╰─ quirements_management/requirements [1282]
          │                    │                │   │  │                                      ╰─ /import_csv [1283]
          │                    │                │   │  │                                                   ╰─ /authorize [1284]
          │                    │                │   │  ├─ aw/
          │                    │                │   │  │    ╰─ {id:*} [1260]
          │                    │                │   │  ╰─ unners [1287]
          │                    │                │   │          ╰─ /
          │                    │                │   │             ├─ new [1293]
          │                    │                │   │             ├─ toggle_
          │                    │                │   │             │        ├─ group_runners [1294]
          │                    │                │   │             │        ╰─ shared_runners [1295]
          │                    │                │   │             ╰─ {id} [1288]
          │                    │                │   │                   ╰─ /
          │                    │                │   │                      ├─ edit [1289]
          │                    │                │   │                      ├─ pause [1290]
          │                    │                │   │                      ╰─ re
          │                    │                │   │                          ├─ gister [1291]
          │                    │                │   │                          ╰─ sume [1292]
          │                    │                │   ├─ p
          │                    │                │   │  ├─ lanning_hierarchy [780]
          │                    │                │   │  ├─ r
          │                    │                │   │  │  ├─ eview
          │                    │                │   │  │  │      ├─ _markdown [781]
          │                    │                │   │  │  │      ╰─ /
          │                    │                │   │  │  │         ╰─ {id:*} [855]
          │                    │                │   │  │  ╰─ o
          │                    │                │   │  │     ├─ ject_members [1237]
          │                    │                │   │  │     │             ╰─ /
          │                    │                │   │  │     │                ├─ leave [1241]
          │                    │                │   │  │     │                ├─ request_access [1242]
          │                    │                │   │  │     │                ╰─ {id} [1238]
          │                    │                │   │  │     │                      ╰─ /
          │                    │                │   │  │     │                         ├─ approve_access_request [1239]
          │                    │                │   │  │     │                         ╰─ resend_invite [1240]
          │                    │                │   │  │     ╰─ tected_
          │                    │                │   │  │              ├─ branches [1249]
          │                    │                │   │  │              │         ╰─ /
          │                    │                │   │  │              │            ╰─ {id} [1250]
          │                    │                │   │  │              ├─ environments [1251]
          │                    │                │   │  │              │             ╰─ /
          │                    │                │   │  │              │                ├─ search [1253]
          │                    │                │   │  │              │                ╰─ {id} [1252]
          │                    │                │   │  │              ╰─ tags [1254]
          │                    │                │   │  │                    ╰─ /
          │                    │                │   │  │                       ╰─ {id} [1255]
          │                    │                │   │  ├─ ackage
          │                    │                │   │  │       ├─ _files/
          │                    │                │   │  │       │        ╰─ {id}
          │                    │                │   │  │       │              ╰─ /download [1193]
          │                    │                │   │  │       ╰─ s [1194]
          │                    │                │   │  │          ╰─ /
          │                    │                │   │  │             ╰─ {id} [1195]
          │                    │                │   │  ├─ ipeline
          │                    │                │   │  │        ├─ _schedules [1208]
          │                    │                │   │  │        │           ╰─ /
          │                    │                │   │  │        │              ├─ new [1213]
          │                    │                │   │  │        │              ╰─ {id} [1209]
          │                    │                │   │  │        │                    ╰─ /
          │                    │                │   │  │        │                       ├─ edit [1210]
          │                    │                │   │  │        │                       ├─ play [1211]
          │                    │                │   │  │        │                       ╰─ take_ownership [1212]
          │                    │                │   │  │        ╰─ s [1214]
          │                    │                │   │  │           ╰─ /
          │                    │                │   │  │              ├─ charts [1229]
          │                    │                │   │  │              ├─ latest [1230]
          │                    │                │   │  │              ├─ new [1231]
          │                    │                │   │  │              ├─ settings [1236]
          │                    │                │   │  │              ├─ {id} [1216]
          │                    │                │   │  │              │     ╰─ /
          │                    │                │   │  │              │        ├─ builds [1217]
          │                    │                │   │  │              │        ├─ c
          │                    │                │   │  │              │        │  ├─ ancel [1218]
          │                    │                │   │  │              │        │  ╰─ odequality_report [1219]
          │                    │                │   │  │              │        ├─ d
          │                    │                │   │  │              │        │  ├─ ag [1220]
          │                    │                │   │  │              │        │  ╰─ ownloadable_artifacts [1221]
          │                    │                │   │  │              │        ├─ failures [1222]
          │                    │                │   │  │              │        ├─ licenses [1223]
          │                    │                │   │  │              │        ├─ retry [1224]
          │                    │                │   │  │              │        ├─ s
          │                    │                │   │  │              │        │  ├─ ecurity [1225]
          │                    │                │   │  │              │        │  ╰─ ta
          │                    │                │   │  │              │        │      ├─ ge [1226]
          │                    │                │   │  │              │        │      ╰─ tus [1227]
          │                    │                │   │  │              │        ╰─ test_report [1228]
          │                    │                │   │  │              ├─ {pipeline_id}
          │                    │                │   │  │              │              ╰─ /
          │                    │                │   │  │              │                 ├─ validate_account [1232]
          │                    │                │   │  │              │                 ├─ stages/
          │                    │                │   │  │              │                 │        ╰─ {stage_name}
          │                    │                │   │  │              │                 │                      ╰─ /play_manual [1233]
          │                    │                │   │  │              │                 ╰─ tests/
          │                    │                │   │  │              │                         ├─ summary [1235]
          │                    │                │   │  │              │                         ╰─ {suite_name} [1234]
          │                    │                │   │  │              ╰─ {ref:*}
          │                    │                │   │  │                       ╰─ /latest [1215]
          │                    │                │   │  ╰─ ush_rules/
          │                    │                │   │              ╰─ {id} [1256]
          │                    │                │   ├─ a
          │                    │                │   │  ├─ lert_management [785]
          │                    │                │   │  │                ╰─ /
          │                    │                │   │  │                   ╰─ {id} [786]
          │                    │                │   │  │                         ╰─ /details [787]
          │                    │                │   │  │                                   ╰─ /
          │                    │                │   │  │                                      ╰─ {page:*} [788]
          │                    │                │   │  ├─ nalytics/
          │                    │                │   │  │          ├─ code_reviews [793]
          │                    │                │   │  │          ├─ value_stream_analytics [794]
          │                    │                │   │  │          │                       ╰─ /
          │                    │                │   │  │          │                          ├─ value_streams [803]
          │                    │                │   │  │          │                          │              ╰─ /
          │                    │                │   │  │          │                          │                 ├─ new [806]
          │                    │                │   │  │          │                          │                 ├─ {value_stream_id}
          │                    │                │   │  │          │                          │                 │                  ╰─ /stages [795]
          │                    │                │   │  │          │                          │                 │                           ╰─ /
          │                    │                │   │  │          │                          │                 │                              ╰─ {id}
          │                    │                │   │  │          │                          │                 │                                    ╰─ /
          │                    │                │   │  │          │                          │                 │                                       ├─ average [796]
          │                    │                │   │  │          │                          │                 │                                       │        ╰─ _duration_chart [797]
          │                    │                │   │  │          │                          │                 │                                       ├─ count [798]
          │                    │                │   │  │          │                          │                 │                                       ├─ median [799]
          │                    │                │   │  │          │                          │                 │                                       ╰─ records [800]
          │                    │                │   │  │          │                          │                 ╰─ {id} [804]
          │                    │                │   │  │          │                          │                       ╰─ /edit [805]
          │                    │                │   │  │          │                          ├─ summary [801]
          │                    │                │   │  │          │                          ╰─ time_summary [802]
          │                    │                │   │  │          ├─ dashboards [807]
          │                    │                │   │  │          │           ╰─ /
          │                    │                │   │  │          │              ╰─ {vueroute:*} [808]
          │                    │                │   │  │          ├─ issues_analytics [809]
          │                    │                │   │  │          ╰─ merge_request_analytics [810]
          │                    │                │   │  ├─ pprover
          │                    │                │   │  │        ├─ _groups/
          │                    │                │   │  │        │         ╰─ {id} [811]
          │                    │                │   │  │        ╰─ s/
          │                    │                │   │  │            ╰─ {id} [813]
          │                    │                │   │  ├─ r
          │                    │                │   │  │  ├─ tifacts [816]
          │                    │                │   │  │  │        ╰─ /
          │                    │                │   │  │  │           ╰─ {id} [817]
          │                    │                │   │  │  ╰─ chive/
          │                    │                │   │  │          ╰─ {id:*}
          │                    │                │   │  │                  ╰─ .
          │                    │                │   │  │                     ╰─ {format} [1279]
          │                    │                │   │  ├─ u
          │                    │                │   │  │  ├─ dit_events [826]
          │                    │                │   │  │  ╰─ to
          │                    │                │   │  │      ├─ complete_sources/
          │                    │                │   │  │      │                  ├─ co
          │                    │                │   │  │      │                  │   ├─ mmands [827]
          │                    │                │   │  │      │                  │   ╰─ ntacts [828]
          │                    │                │   │  │      │                  ├─ epics [829]
          │                    │                │   │  │      │                  ├─ i
          │                    │                │   │  │      │                  │  ├─ ssues [830]
          │                    │                │   │  │      │                  │  ╰─ terations [831]
          │                    │                │   │  │      │                  ├─ labels [832]
          │                    │                │   │  │      │                  ├─ m
          │                    │                │   │  │      │                  │  ├─ e
          │                    │                │   │  │      │                  │  │  ├─ mbers [833]
          │                    │                │   │  │      │                  │  │  ╰─ rge_requests [834]
          │                    │                │   │  │      │                  │  ╰─ ilestones [835]
          │                    │                │   │  │      │                  ├─ snippets [836]
          │                    │                │   │  │      │                  ├─ vulnerabilities [837]
          │                    │                │   │  │      │                  ╰─ wikis [838]
          │                    │                │   │  │      ╰─ mations [839]
          │                    │                │   │  ├─ vatar [840]
          │                    │                │   │  ╰─ ws/configuration [841]
          │                    │                │   ├─ m
          │                    │                │   │  ├─ e
          │                    │                │   │  │  ├─ rge
          │                    │                │   │  │  │    ├─ _
          │                    │                │   │  │  │    │  ├─ requests [1087]
          │                    │                │   │  │  │    │  │         ╰─ /
          │                    │                │   │  │  │    │  │            ├─ bulk_update [1124]
          │                    │                │   │  │  │    │  │            ├─ diff_for_path [1125]
          │                    │                │   │  │  │    │  │            ├─ export_csv [1126]
          │                    │                │   │  │  │    │  │            ├─ new [1132]
          │                    │                │   │  │  │    │  │            │    ╰─ /
          │                    │                │   │  │  │    │  │            │       ├─ branch_
          │                    │                │   │  │  │    │  │            │       │        ├─ from [1133]
          │                    │                │   │  │  │    │  │            │       │        ╰─ to [1134]
          │                    │                │   │  │  │    │  │            │       ├─ diff
          │                    │                │   │  │  │    │  │            │       │     ├─ _for_path [1135]
          │                    │                │   │  │  │    │  │            │       │     ╰─ s [1136]
          │                    │                │   │  │  │    │  │            │       ├─ pipelines [1137]
          │                    │                │   │  │  │    │  │            │       ╰─ target_projects [1138]
          │                    │                │   │  │  │    │  │            ├─ {merge_request_id}
          │                    │                │   │  │  │    │  │            │                   ╰─ /
          │                    │                │   │  │  │    │  │            │                      ├─ approver
          │                    │                │   │  │  │    │  │            │                      │         ├─ _groups/
          │                    │                │   │  │  │    │  │            │                      │         │         ╰─ {id} [812]
          │                    │                │   │  │  │    │  │            │                      │         ╰─ s [814]
          │                    │                │   │  │  │    │  │            │                      │            ╰─ /
          │                    │                │   │  │  │    │  │            │                      │               ╰─ {id} [815]
          │                    │                │   │  │  │    │  │            │                      ╰─ drafts [1145]
          │                    │                │   │  │  │    │  │            │                              ╰─ /
          │                    │                │   │  │  │    │  │            │                                 ├─ discard [1147]
          │                    │                │   │  │  │    │  │            │                                 ├─ publish [1148]
          │                    │                │   │  │  │    │  │            │                                 ╰─ {id} [1146]
          │                    │                │   │  │  │    │  │            ╰─ {id} [1088]
          │                    │                │   │  │  │    │  │                  ╰─ /
          │                    │                │   │  │  │    │  │                     ├─ a
          │                    │                │   │  │  │    │  │                     │  ├─ ccessibility_reports [1089]
          │                    │                │   │  │  │    │  │                     │  ├─ pi_fuzzing_reports [1090]
          │                    │                │   │  │  │    │  │                     │  ╰─ ssign_related_issues [1091]
          │                    │                │   │  │  │    │  │                     ├─ c
          │                    │                │   │  │  │    │  │                     │  ├─ a
          │                    │                │   │  │  │    │  │                     │  │  ├─ ncel_auto_merge [1092]
          │                    │                │   │  │  │    │  │                     │  │  ╰─ ched_widget [1130]
          │                    │                │   │  │  │    │  │                     │  ├─ i_environments_status [1093]
          │                    │                │   │  │  │    │  │                     │  ╰─ o
          │                    │                │   │  │  │    │  │                     │     ├─ dequality_
          │                    │                │   │  │  │    │  │                     │     │           ├─ mr_diff_reports [1094]
          │                    │                │   │  │  │    │  │                     │     │           ╰─ reports [1095]
          │                    │                │   │  │  │    │  │                     │     ├─ mmit
          │                    │                │   │  │  │    │  │                     │     │     ├─ _change_content [1096]
          │                    │                │   │  │  │    │  │                     │     │     ╰─ s [1097]
          │                    │                │   │  │  │    │  │                     │     ├─ n
          │                    │                │   │  │  │    │  │                     │     │  ├─ t
          │                    │                │   │  │  │    │  │                     │     │  │  ├─ ainer_scanning_reports [1098]
          │                    │                │   │  │  │    │  │                     │     │  │  ╰─ ext_commits [1099]
          │                    │                │   │  │  │    │  │                     │     │  ╰─ flict
          │                    │                │   │  │  │    │  │                     │     │         ├─ _for_path [1127]
          │                    │                │   │  │  │    │  │                     │     │         ╰─ s [1128]
          │                    │                │   │  │  │    │  │                     │     ╰─ verage_
          │                    │                │   │  │  │    │  │                     │              ├─ fuzzing_reports [1100]
          │                    │                │   │  │  │    │  │                     │              ╰─ reports [1101]
          │                    │                │   │  │  │    │  │                     ├─ d
          │                    │                │   │  │  │    │  │                     │  ├─ ast_reports [1102]
          │                    │                │   │  │  │    │  │                     │  ├─ e
          │                    │                │   │  │  │    │  │                     │  │  ├─ pendency_scanning_reports [1103]
          │                    │                │   │  │  │    │  │                     │  │  ╰─ scriptions/
          │                    │                │   │  │  │    │  │                     │  │               ╰─ {version_id} [1104]
          │                    │                │   │  │  │    │  │                     │  │                             ╰─ /diff [1105]
          │                    │                │   │  │  │    │  │                     │  ╰─ i
          │                    │                │   │  │  │    │  │                     │     ├─ scussions [1106]
          │                    │                │   │  │  │    │  │                     │     ╰─ ff
          │                    │                │   │  │  │    │  │                     │         ├─ _
          │                    │                │   │  │  │    │  │                     │         │  ├─ by_file_hash/
          │                    │                │   │  │  │    │  │                     │         │  │              ╰─ {file_hash} [1139]
          │                    │                │   │  │  │    │  │                     │         │  ╰─ for_path [1140]
          │                    │                │   │  │  │    │  │                     │         ╰─ s [1141]
          │                    │                │   │  │  │    │  │                     │            ╰─ _
          │                    │                │   │  │  │    │  │                     │               ├─ batch [1142]
          │                    │                │   │  │  │    │  │                     │               ├─ metadata [1143]
          │                    │                │   │  │  │    │  │                     │               ╰─ stream [1144]
          │                    │                │   │  │  │    │  │                     ├─ e
          │                    │                │   │  │  │    │  │                     │  ├─ dit [1107]
          │                    │                │   │  │  │    │  │                     │  ╰─ xposed_artifacts [1108]
          │                    │                │   │  │  │    │  │                     ├─ license_scanning_reports [1109]
          │                    │                │   │  │  │    │  │                     │                         ╰─ _collapsed [1110]
          │                    │                │   │  │  │    │  │                     ├─ me
          │                    │                │   │  │  │    │  │                     │   ├─ rge [1111]
          │                    │                │   │  │  │    │  │                     │   ╰─ trics_reports [1112]
          │                    │                │   │  │  │    │  │                     ├─ pipeline
          │                    │                │   │  │  │    │  │                     │         ├─ _status [1113]
          │                    │                │   │  │  │    │  │                     │         ╰─ s [1114]
          │                    │                │   │  │  │    │  │                     ├─ re
          │                    │                │   │  │  │    │  │                     │   ├─ base [1115]
          │                    │                │   │  │  │    │  │                     │   ├─ move_wip [1116]
          │                    │                │   │  │  │    │  │                     │   ╰─ solve_conflicts [1129]
          │                    │                │   │  │  │    │  │                     ├─ s
          │                    │                │   │  │  │    │  │                     │  ├─ a
          │                    │                │   │  │  │    │  │                     │  │  ├─ st_reports [1117]
          │                    │                │   │  │  │    │  │                     │  │  ╰─ ml_approval [1149]
          │                    │                │   │  │  │    │  │                     │  ╰─ ec
          │                    │                │   │  │  │    │  │                     │      ├─ ret_detection_reports [1118]
          │                    │                │   │  │  │    │  │                     │      ╰─ urity_reports [1119]
          │                    │                │   │  │  │    │  │                     ├─ t
          │                    │                │   │  │  │    │  │                     │  ├─ e
          │                    │                │   │  │  │    │  │                     │  │  ├─ rraform_reports [1120]
          │                    │                │   │  │  │    │  │                     │  │  ╰─ st_reports [1121]
          │                    │                │   │  │  │    │  │                     │  ╰─ oggle_
          │                    │                │   │  │  │    │  │                     │          ├─ award_emoji [1122]
          │                    │                │   │  │  │    │  │                     │          ╰─ subscription [1123]
          │                    │                │   │  │  │    │  │                     ╰─ widget [1131]
          │                    │                │   │  │  │    │  ╰─ trains [1150]
          │                    │                │   │  │  │    ╰─ d_branches [865]
          │                    │                │   │  │  ╰─ trics [1151]
          │                    │                │   │  │         ╰─ /
          │                    │                │   │  │            ╰─ {id} [1152]
          │                    │                │   │  ├─ attermost [1085]
          │                    │                │   │  │          ╰─ /new [1086]
          │                    │                │   │  ├─ i
          │                    │                │   │  │  ├─ lestones [1153]
          │                    │                │   │  │  │         ╰─ /
          │                    │                │   │  │  │            ├─ new [1161]
          │                    │                │   │  │  │            ╰─ {id} [1154]
          │                    │                │   │  │  │                  ╰─ /
          │                    │                │   │  │  │                     ├─ edit [1155]
          │                    │                │   │  │  │                     ├─ issues [1156]
          │                    │                │   │  │  │                     ├─ labels [1157]
          │                    │                │   │  │  │                     ├─ merge_requests [1158]
          │                    │                │   │  │  │                     ╰─ p
          │                    │                │   │  │  │                        ├─ articipants [1159]
          │                    │                │   │  │  │                        ╰─ romote [1160]
          │                    │                │   │  │  ╰─ rror [1162]
          │                    │                │   │  │        ╰─ /
          │                    │                │   │  │           ├─ ssh_host_keys [1163]
          │                    │                │   │  │           ╰─ update_now [1164]
          │                    │                │   │  ╰─ l/
          │                    │                │   │      ├─ agents [1165]
          │                    │                │   │      │       ╰─ /
          │                    │                │   │      │          ├─ new [1172]
          │                    │                │   │      │          ├─ {id} [1170]
          │                    │                │   │      │          │     ╰─ /edit [1171]
          │                    │                │   │      │          ├─ {vueroute:*}
          │                    │                │   │      │          │             ╰─ /
          │                    │                │   │      │          │                ├─ new [1169]
          │                    │                │   │      │          │                ╰─ {id} [1167]
          │                    │                │   │      │          │                      ╰─ /edit [1168]
          │                    │                │   │      │          ╰─ {vueroute:*} [1166]
          │                    │                │   │      ├─ candidates/
          │                    │                │   │      │            ╰─ {iid} [1173]
          │                    │                │   │      ├─ experiments [1174]
          │                    │                │   │      │            ╰─ /
          │                    │                │   │      │               ╰─ {iid} [1175]
          │                    │                │   │      ╰─ models [1177]
          │                    │                │   │              ╰─ /
          │                    │                │   │                 ├─ new [1179]
          │                    │                │   │                 ├─ {model_model_id}
          │                    │                │   │                 │                 ╰─ /versions/
          │                    │                │   │                 │                             ╰─ {model_version_id} [1176]
          │                    │                │   │                 ╰─ {model_id} [1178]
          │                    │                │   ├─ jobs [1058]
          │                    │                │   │     ╰─ /
          │                    │                │   │        ├─ artifacts/
          │                    │                │   │        │           ╰─ {ref_name_and_path:*} [825]
          │                    │                │   │        ├─ {job_id}
          │                    │                │   │        │         ╰─ /artifacts/
          │                    │                │   │        │                      ├─ browse [818]
          │                    │                │   │        │                      │       ╰─ /
          │                    │                │   │        │                      │          ╰─ {path:*} [819]
          │                    │                │   │        │                      ├─ download [820]
          │                    │                │   │        │                      ├─ external_file/
          │                    │                │   │        │                      │               ╰─ {path:*} [821]
          │                    │                │   │        │                      ├─ file/
          │                    │                │   │        │                      │      ╰─ {path:*} [822]
          │                    │                │   │        │                      ├─ keep [823]
          │                    │                │   │        │                      ╰─ raw/
          │                    │                │   │        │                            ╰─ {path:*} [824]
          │                    │                │   │        ╰─ {id} [1059]
          │                    │                │   │              ╰─ /
          │                    │                │   │                 ├─ cancel [1060]
          │                    │                │   │                 ├─ erase [1061]
          │                    │                │   │                 ├─ p
          │                    │                │   │                 │  ├─ lay [1062]
          │                    │                │   │                 │  ╰─ roxy [1063]
          │                    │                │   │                 │        ╰─ .ws/authorize [1064]
          │                    │                │   │                 ├─ r
          │                    │                │   │                 │  ├─ aw [1065]
          │                    │                │   │                 │  ╰─ etry [1066]
          │                    │                │   │                 ├─ status [1067]
          │                    │                │   │                 ├─ t
          │                    │                │   │                 │  ├─ e
          │                    │                │   │                 │  │  ├─ rminal [1068]
          │                    │                │   │                 │  │  │       ╰─ .ws/authorize [1069]
          │                    │                │   │                 │  │  ╰─ st_report_summary [1070]
          │                    │                │   │                 │  ╰─ race [1071]
          │                    │                │   │                 ├─ unschedule [1072]
          │                    │                │   │                 ╰─ viewer [1073]
          │                    │                │   ├─ b
          │                    │                │   │  ├─ adges/release [842]
          │                    │                │   │  ├─ l
          │                    │                │   │  │  ├─ ame
          │                    │                │   │  │  │    ├─ _page/
          │                    │                │   │  │  │    │       ╰─ {id:*} [846]
          │                    │                │   │  │  │    ╰─ /
          │                    │                │   │  │  │       ├─ {id:*}
          │                    │                │   │  │  │       │       ╰─ /streaming [848]
          │                    │                │   │  │  │       ╰─ {id:*} [847]
          │                    │                │   │  │  ╰─ ob/
          │                    │                │   │  │       ├─ {id:*}
          │                    │                │   │  │       │       ╰─ /diff [851]
          │                    │                │   │  │       ╰─ {id:*} [850]
          │                    │                │   │  ├─ oards [858]
          │                    │                │   │  │      ╰─ /
          │                    │                │   │  │         ╰─ {id} [859]
          │                    │                │   │  ╰─ ranches [860]
          │                    │                │   │           ╰─ /
          │                    │                │   │              ├─ diverging_commit_counts [863]
          │                    │                │   │              ├─ new [864]
          │                    │                │   │              ├─ {id} [861]
          │                    │                │   │              ╰─ {state} [862]
          │                    │                │   ├─ c
          │                    │                │   │  ├─ reate
          │                    │                │   │  │      ├─ /
          │                    │                │   │  │      │  ╰─ {id:*} [852]
          │                    │                │   │  │      ╰─ _dir/
          │                    │                │   │  │             ╰─ {id:*} [1376]
          │                    │                │   │  ├─ i/
          │                    │                │   │  │   ├─ daily_build_group_report_results [875]
          │                    │                │   │  │   ├─ lint [876]
          │                    │                │   │  │   ├─ editor [877]
          │                    │                │   │  │   ╰─ prometheus_metrics/histograms [878]
          │                    │                │   │  ├─ luster
          │                    │                │   │  │       ├─ _agents/
          │                    │                │   │  │       │         ╰─ {name} [879]
          │                    │                │   │  │       ╰─ s [880]
          │                    │                │   │  │          ╰─ /
          │                    │                │   │  │             ├─ c
          │                    │                │   │  │             │  ├─ onnect [887]
          │                    │                │   │  │             │  ╰─ reate_user [888]
          │                    │                │   │  │             ├─ new_cluster_docs [889]
          │                    │                │   │  │             ├─ {id} [881]
          │                    │                │   │  │             │     ╰─ /
          │                    │                │   │  │             │        ├─ cl
          │                    │                │   │  │             │        │   ├─ ear_cache [882]
          │                    │                │   │  │             │        │   ╰─ uster_status [883]
          │                    │                │   │  │             │        ├─ environments [884]
          │                    │                │   │  │             │        ╰─ metrics [885]
          │                    │                │   │  │             │                 ╰─ _dashboard [886]
          │                    │                │   │  │             ╰─ {cluster_id}
          │                    │                │   │  │                           ╰─ /integration/create_or_update [890]
          │                    │                │   │  ├─ om
          │                    │                │   │  │   ├─ m
          │                    │                │   │  │   │  ├─ ent_templates [891]
          │                    │                │   │  │   │  │              ╰─ /
          │                    │                │   │  │   │  │                 ╰─ {id} [892]
          │                    │                │   │  │   │  ╰─ it
          │                    │                │   │  │   │      ├─ /
          │                    │                │   │  │   │      │  ╰─ {id} [893]
          │                    │                │   │  │   │      │        ╰─ /
          │                    │                │   │  │   │      │           ├─ branches [894]
          │                    │                │   │  │   │      │           ├─ cherry_pick [895]
          │                    │                │   │  │   │      │           ├─ diff_f
          │                    │                │   │  │   │      │           │       ├─ iles [896]
          │                    │                │   │  │   │      │           │       ╰─ or_path [897]
          │                    │                │   │  │   │      │           ├─ merge_requests [898]
          │                    │                │   │  │   │      │           ├─ pipelines [899]
          │                    │                │   │  │   │      │           ╰─ revert [900]
          │                    │                │   │  │   │      ╰─ s [901]
          │                    │                │   │  │   │         ╰─ /
          │                    │                │   │  │   │            ├─ {id:*}
          │                    │                │   │  │   │            │       ╰─ /signatures [903]
          │                    │                │   │  │   │            ╰─ {id:*} [902]
          │                    │                │   │  │   ╰─ p
          │                    │                │   │  │      ├─ are [904]
          │                    │                │   │  │      │    ╰─ /
          │                    │                │   │  │      │       ├─ diff_for_path [906]
          │                    │                │   │  │      │       ├─ signatures [907]
          │                    │                │   │  │      │       ╰─ {from}
          │                    │                │   │  │      │               ╰─ ...
          │                    │                │   │  │      │                    ╰─ {to} [905]
          │                    │                │   │  │      ╰─ liance_frameworks [908]
          │                    │                │   │  ╰─ adences [1044]
          │                    │                │   │           ╰─ /
          │                    │                │   │              ├─ new [1055]
          │                    │                │   │              ├─ {id} [1051]
          │                    │                │   │              │     ╰─ /edit [1052]
          │                    │                │   │              ├─ {iteration_cadence_id}
          │                    │                │   │              │                       ╰─ /iterations [1053]
          │                    │                │   │              │                                    ╰─ /
          │                    │                │   │              │                                       ╰─ {id} [1054]
          │                    │                │   │              ├─ {vueroute:*}
          │                    │                │   │              │             ╰─ /
          │                    │                │   │              │                ├─ new [1050]
          │                    │                │   │              │                ├─ {id} [1046]
          │                    │                │   │              │                │     ╰─ /edit [1047]
          │                    │                │   │              │                ╰─ {iteration_cadence_id}
          │                    │                │   │              │                                        ╰─ /iterations [1048]
          │                    │                │   │              │                                                     ╰─ /
          │                    │                │   │              │                                                        ╰─ {id} [1049]
          │                    │                │   │              ╰─ {vueroute:*} [1045]
          │                    │                │   ├─ e
          │                    │                │   │  ├─ dit/
          │                    │                │   │  │     ╰─ {id:*} [853]
          │                    │                │   │  ├─ nvironments [939]
          │                    │                │   │  │            ╰─ /
          │                    │                │   │  │               ├─ folders/
          │                    │                │   │  │               │         ╰─ {id:*} [948]
          │                    │                │   │  │               ├─ new [949]
          │                    │                │   │  │               ├─ search [950]
          │                    │                │   │  │               ├─ {environment_id}
          │                    │                │   │  │               │                 ╰─ /deployments [929]
          │                    │                │   │  │               │                               ╰─ /
          │                    │                │   │  │               │                                  ╰─ {id} [930]
          │                    │                │   │  │               │                                        ╰─ /
          │                    │                │   │  │               │                                           ├─ additional_metrics [931]
          │                    │                │   │  │               │                                           ╰─ metrics [932]
          │                    │                │   │  │               ╰─ {id} [940]
          │                    │                │   │  │                     ╰─ /
          │                    │                │   │  │                        ├─ cancel_auto_stop [941]
          │                    │                │   │  │                        ├─ edit [942]
          │                    │                │   │  │                        ├─ k8s [943]
          │                    │                │   │  │                        │    ╰─ /
          │                    │                │   │  │                        │       ╰─ {vueroute:*} [944]
          │                    │                │   │  │                        ├─ stop [945]
          │                    │                │   │  │                        ├─ terminal [946]
          │                    │                │   │  │                        │         ╰─ .ws/authorize [947]
          │                    │                │   │  │                        ╰─ prometheus/api/v1/
          │                    │                │   │  │                                            ╰─ {proxy_path:*} [951]
          │                    │                │   │  ├─ rror_tracking [952]
          │                    │                │   │  │              ╰─ /
          │                    │                │   │  │                 ├─ projects [955]
          │                    │                │   │  │                 ╰─ {issue_id} [953]
          │                    │                │   │  │                             ╰─ /
          │                    │                │   │  │                                ├─ details [954]
          │                    │                │   │  │                                ╰─ stack_trace [956]
          │                    │                │   │  ╰─ scalation_policies [1002]
          │                    │                │   ├─ ne
          │                    │                │   │   ├─ w/
          │                    │                │   │   │   ╰─ {id:*} [854]
          │                    │                │   │   ╰─ twork/
          │                    │                │   │           ╰─ {id} [1180]
          │                    │                │   ├─ u
          │                    │                │   │  ├─ pdate/
          │                    │                │   │  │       ╰─ {id:*} [856]
          │                    │                │   │  ╰─ sage_quotas [1384]
          │                    │                │   ├─ w
          │                    │                │   │  ├─ ikis [1395]
          │                    │                │   │  │     ╰─ /
          │                    │                │   │  │        ├─ -/confluence [909]
          │                    │                │   │  │        ├─ git_access [1402]
          │                    │                │   │  │        ├─ new [1403]
          │                    │                │   │  │        ├─ pages [1404]
          │                    │                │   │  │        ├─ templates [1405]
          │                    │                │   │  │        ├─ {id:*}
          │                    │                │   │  │        │       ╰─ /
          │                    │                │   │  │        │          ├─ diff [1397]
          │                    │                │   │  │        │          ├─ edit [1398]
          │                    │                │   │  │        │          ├─ history [1399]
          │                    │                │   │  │        │          ├─ preview_markdown [1400]
          │                    │                │   │  │        │          ╰─ raw [1401]
          │                    │                │   │  │        ╰─ {id:*} [1396]
          │                    │                │   │  ╰─ ork_items/
          │                    │                │   │              ├─ import_csv [1409]
          │                    │                │   │              │           ╰─ /authorize [1410]
          │                    │                │   │              ╰─ {iid} [1406]
          │                    │                │   │                     ╰─ /designs [1407]
          │                    │                │   │                               ╰─ /
          │                    │                │   │                                  ╰─ {vueroute:*} [1408]
          │                    │                │   ├─ v
          │                    │                │   │  ├─ a
          │                    │                │   │  │  ├─ lue_stream_analytics [910]
          │                    │                │   │  │  │                     ╰─ /events/
          │                    │                │   │  │  │                               ├─ code [911]
          │                    │                │   │  │  │                               ├─ issue [912]
          │                    │                │   │  │  │                               ├─ p
          │                    │                │   │  │  │                               │  ├─ lan [913]
          │                    │                │   │  │  │                               │  ╰─ roduction [914]
          │                    │                │   │  │  │                               ├─ review [915]
          │                    │                │   │  │  │                               ├─ staging [916]
          │                    │                │   │  │  │                               ╰─ test [917]
          │                    │                │   │  │  ╰─ riables [1385]
          │                    │                │   │  ╰─ ulnerability_feedback [1386]
          │                    │                │   │                         ╰─ /
          │                    │                │   │                            ├─ count [1388]
          │                    │                │   │                            ╰─ {id} [1387]
          │                    │                │   ├─ de
          │                    │                │   │   ├─ p
          │                    │                │   │   │  ├─ endencies [918]
          │                    │                │   │   │  ╰─ loy_
          │                    │                │   │   │        ├─ keys [919]
          │                    │                │   │   │        │     ╰─ /
          │                    │                │   │   │        │        ├─ available_p
          │                    │                │   │   │        │        │            ├─ roject_keys [924]
          │                    │                │   │   │        │        │            ╰─ ublic_keys [925]
          │                    │                │   │   │        │        ├─ enabled_keys [926]
          │                    │                │   │   │        │        ├─ new [927]
          │                    │                │   │   │        │        ╰─ {id} [920]
          │                    │                │   │   │        │              ╰─ /
          │                    │                │   │   │        │                 ├─ disable [921]
          │                    │                │   │   │        │                 ╰─ e
          │                    │                │   │   │        │                    ├─ dit [922]
          │                    │                │   │   │        │                    ╰─ nable [923]
          │                    │                │   │   │        ╰─ tokens/
          │                    │                │   │   │                 ╰─ {id}
          │                    │                │   │   │                       ╰─ /revoke [928]
          │                    │                │   │   ╰─ sign_management/designs/
          │                    │                │   │                             ╰─ {design_id}
          │                    │                │   │                                          ╰─ /
          │                    │                │   │                                             ├─ r
          │                    │                │   │                                             │  ├─ aw_image [934]
          │                    │                │   │                                             │  ╰─ esized_image/
          │                    │                │   │                                             │                 ╰─ {id} [936]
          │                    │                │   │                                             ╰─ {sha}
          │                    │                │   │                                                    ╰─ /r
          │                    │                │   │                                                        ├─ aw_image [933]
          │                    │                │   │                                                        ╰─ esized_image/
          │                    │                │   │                                                                       ╰─ {id} [935]
          │                    │                │   ├─ f
          │                    │                │   │  ├─ eature_flags [959]
          │                    │                │   │  │             ├─ /
          │                    │                │   │  │             │  ├─ new [962]
          │                    │                │   │  │             │  ├─ {feature_flag_iid}
          │                    │                │   │  │             │  │                   ╰─ /issues [957]
          │                    │                │   │  │             │  │                            ╰─ /
          │                    │                │   │  │             │  │                               ╰─ {id} [958]
          │                    │                │   │  │             │  ╰─ {iid} [960]
          │                    │                │   │  │             │         ╰─ /edit [961]
          │                    │                │   │  │             ╰─ _
          │                    │                │   │  │                ├─ client/reset_token [963]
          │                    │                │   │  │                ╰─ user_lists [964]
          │                    │                │   │  │                            ╰─ /
          │                    │                │   │  │                               ├─ new [967]
          │                    │                │   │  │                               ╰─ {iid} [965]
          │                    │                │   │  │                                      ╰─ /edit [966]
          │                    │                │   │  ├─ i
          │                    │                │   │  │  ├─ les/
          │                    │                │   │  │  │     ╰─ {id:*} [968]
          │                    │                │   │  │  ╰─ nd_file/
          │                    │                │   │  │            ╰─ {id:*} [969]
          │                    │                │   │  ╰─ orks [970]
          │                    │                │   │        ╰─ /new [971]
          │                    │                │   ├─ g
          │                    │                │   │  ├─ oogle_cloud/
          │                    │                │   │  │             ├─ artifact_registry [972]
          │                    │                │   │  │             │                  ╰─ /projects/
          │                    │                │   │  │             │                              ╰─ {project}
          │                    │                │   │  │             │                                         ╰─ /locations/
          │                    │                │   │  │             │                                                      ╰─ {location}
          │                    │                │   │  │             │                                                                  ╰─ /repositories/
          │                    │                │   │  │             │                                                                                  ╰─ {repository}
          │                    │                │   │  │             │                                                                                                ╰─ /dockerImages/
          │                    │                │   │  │             │                                                                                                                ╰─ {image} [973]
          │                    │                │   │  │             ├─ configuration [974]
          │                    │                │   │  │             ├─ d
          │                    │                │   │  │             │  ├─ atabases [975]
          │                    │                │   │  │             │  │         ╰─ /new/
          │                    │                │   │  │             │  │                ╰─ {product} [976]
          │                    │                │   │  │             │  ╰─ eployments [977]
          │                    │                │   │  │             │              ╰─ /cloud_
          │                    │                │   │  │             │                       ├─ run [978]
          │                    │                │   │  │             │                       ╰─ storage [979]
          │                    │                │   │  │             ├─ gcp_regions [980]
          │                    │                │   │  │             ├─ revoke_oauth [981]
          │                    │                │   │  │             ╰─ service_accounts [982]
          │                    │                │   │  ╰─ r
          │                    │                │   │     ├─ aphs/
          │                    │                │   │     │      ╰─ {id} [983]
          │                    │                │   │     │            ╰─ /
          │                    │                │   │     │               ├─ c
          │                    │                │   │     │               │  ├─ harts [984]
          │                    │                │   │     │               │  ├─ i [985]
          │                    │                │   │     │               │  ╰─ ommits [986]
          │                    │                │   │     │               ╰─ languages [987]
          │                    │                │   │     ╰─ oup_links/
          │                    │                │   │                 ╰─ {id} [988]
          │                    │                │   ├─ h
          │                    │                │   │  ├─ arbor/repositories [990]
          │                    │                │   │  │                   ╰─ /
          │                    │                │   │  │                      ├─ {repository_id}
          │                    │                │   │  │                      │                ╰─ /artifacts [989]
          │                    │                │   │  │                      │                            ╰─ /
          │                    │                │   │  │                      │                               ╰─ {artifact_id}
          │                    │                │   │  │                      │                                              ╰─ /tags [992]
          │                    │                │   │  │                      ╰─ {id} [991]
          │                    │                │   │  ╰─ ooks [995]
          │                    │                │   │        ╰─ /
          │                    │                │   │           ├─ {hook_id}
          │                    │                │   │           │          ╰─ /hook_logs/
          │                    │                │   │           │                       ╰─ {id} [993]
          │                    │                │   │           │                             ╰─ /retry [994]
          │                    │                │   │           ╰─ {id} [996]
          │                    │                │   │                 ╰─ /
          │                    │                │   │                    ├─ edit [997]
          │                    │                │   │                    ╰─ test [998]
          │                    │                │   ├─ i
          │                    │                │   │  ├─ mport [1000]
          │                    │                │   │  │      ╰─ /
          │                    │                │   │  │         ├─ jira [999]
          │                    │                │   │  │         ╰─ new [1001]
          │                    │                │   │  ├─ n
          │                    │                │   │  │  ├─ cident
          │                    │                │   │  │  │       ├─ s [1006]
          │                    │                │   │  │  │       │  ╰─ /integrations/pagerduty [1004]
          │                    │                │   │  │  │       ╰─ _management/timeline_events/preview_markdown [1005]
          │                    │                │   │  │  ╰─ tegrations/
          │                    │                │   │  │               ├─ jira/issues [1011]
          │                    │                │   │  │               │            ╰─ /
          │                    │                │   │  │               │               ╰─ {id} [1012]
          │                    │                │   │  │               ├─ slash_commands [1013]
          │                    │                │   │  │               │               ╰─ /confirm [1014]
          │                    │                │   │  │               ╰─ zentao/issues [1015]
          │                    │                │   │  │                              ╰─ /
          │                    │                │   │  │                                 ╰─ {id} [1016]
          │                    │                │   │  ├─ ssues [1021]
          │                    │                │   │  │      ╰─ /
          │                    │                │   │  │         ├─ i
          │                    │                │   │  │         │  ├─ ncident/
          │                    │                │   │  │         │  │         ╰─ {id} [1007]
          │                    │                │   │  │         │  │               ╰─ /
          │                    │                │   │  │         │  │                  ╰─ {incident_tab} [1008]
          │                    │                │   │  │         │  ╰─ mport_csv [1041]
          │                    │                │   │  │         ├─ bulk_update [1039]
          │                    │                │   │  │         ├─ export_csv [1040]
          │                    │                │   │  │         ├─ new [1042]
          │                    │                │   │  │         ├─ service_desk [1043]
          │                    │                │   │  │         ├─ {issue_id}
          │                    │                │   │  │         │           ╰─ /
          │                    │                │   │  │         │              ├─ feature_flags [1017]
          │                    │                │   │  │         │              │              ╰─ /
          │                    │                │   │  │         │              │                 ╰─ {id} [1018]
          │                    │                │   │  │         │              ╰─ links [1019]
          │                    │                │   │  │         │                     ╰─ /
          │                    │                │   │  │         │                        ╰─ {id} [1020]
          │                    │                │   │  │         ╰─ {id} [1022]
          │                    │                │   │  │               ╰─ /
          │                    │                │   │  │                  ├─ c
          │                    │                │   │  │                  │  ├─ an_create_branch [1024]
          │                    │                │   │  │                  │  ╰─ reate_merge_request [1025]
          │                    │                │   │  │                  ├─ d
          │                    │                │   │  │                  │  ├─ es
          │                    │                │   │  │                  │  │   ├─ criptions/
          │                    │                │   │  │                  │  │   │           ╰─ {version_id} [1026]
          │                    │                │   │  │                  │  │   │                         ╰─ /diff [1027]
          │                    │                │   │  │                  │  │   ╰─ igns [1028]
          │                    │                │   │  │                  │  │         ╰─ /
          │                    │                │   │  │                  │  │            ╰─ {vueroute:*} [1029]
          │                    │                │   │  │                  │  ╰─ iscussions [1030]
          │                    │                │   │  │                  ├─ edit [1031]
          │                    │                │   │  │                  ├─ m
          │                    │                │   │  │                  │  ├─ ark_as_spam [1032]
          │                    │                │   │  │                  │  ╰─ ove [1033]
          │                    │                │   │  │                  ├─ re
          │                    │                │   │  │                  │   ├─ altime_changes [1034]
          │                    │                │   │  │                  │   ├─ lated_branches [1035]
          │                    │                │   │  │                  │   ╰─ order [1036]
          │                    │                │   │  │                  ├─ toggle_
          │                    │                │   │  │                  │        ├─ award_emoji [1037]
          │                    │                │   │  │                  │        ╰─ subscription [1038]
          │                    │                │   │  │                  ╰─ {incident_tab} [1023]
          │                    │                │   │  ╰─ terations [1056]
          │                    │                │   │             ╰─ /
          │                    │                │   │                ╰─ {id} [1057]
          │                    │                │   ├─ on
          │                    │                │   │   ├─ call_schedules [1003]
          │                    │                │   │   ╰─ _demand_scans [1188]
          │                    │                │   │                  ╰─ /
          │                    │                │   │                     ├─ new [1190]
          │                    │                │   │                     ╰─ {id}
          │                    │                │   │                           ╰─ /edit [1189]
          │                    │                │   ├─ l
          │                    │                │   │  ├─ abels [1074]
          │                    │                │   │  │      ╰─ /
          │                    │                │   │  │         ├─ generate [1080]
          │                    │                │   │  │         ├─ new [1081]
          │                    │                │   │  │         ├─ set_priorities [1082]
          │                    │                │   │  │         ╰─ {id} [1075]
          │                    │                │   │  │               ╰─ /
          │                    │                │   │  │                  ├─ edit [1076]
          │                    │                │   │  │                  ├─ promote [1077]
          │                    │                │   │  │                  ├─ remove_priority [1078]
          │                    │                │   │  │                  ╰─ toggle_subscription [1079]
          │                    │                │   │  ├─ earn_gitlab [1083]
          │                    │                │   │  ╰─ ogs [1084]
          │                    │                │   ├─ t
          │                    │                │   │  ├─ erraform [1372]
          │                    │                │   │  │         ╰─ _module_registry [1191]
          │                    │                │   │  │                           ╰─ /
          │                    │                │   │  │                              ╰─ {id} [1192]
          │                    │                │   │  ├─ a
          │                    │                │   │  │  ├─ gs [1364]
          │                    │                │   │  │  │   ╰─ /
          │                    │                │   │  │  │      ├─ new [1366]
          │                    │                │   │  │  │      ╰─ {id} [1365]
          │                    │                │   │  │  ╰─ rget_branch_rules [1367]
          │                    │                │   │  │                     ╰─ /
          │                    │                │   │  │                        ╰─ {id} [1368]
          │                    │                │   │  ╰─ r
          │                    │                │   │     ├─ acing [1374]
          │                    │                │   │     │      ╰─ /
          │                    │                │   │     │         ╰─ {id} [1375]
          │                    │                │   │     ├─ ee/
          │                    │                │   │     │    ╰─ {id:*} [1377]
          │                    │                │   │     ╰─ iggers [1379]
          │                    │                │   │             ╰─ /
          │                    │                │   │                ╰─ {id} [1380]
          │                    │                │   ├─ quality/test_cases [1257]
          │                    │                │   │                   ╰─ /
          │                    │                │   │                      ├─ new [1259]
          │                    │                │   │                      ╰─ {id} [1258]
          │                    │                │   ├─ s
          │                    │                │   │  ├─ e
          │                    │                │   │  │  ├─ c
          │                    │                │   │  │  │  ├─ rets [1296]
          │                    │                │   │  │  │  │     ╰─ /
          │                    │                │   │  │  │  │        ╰─ {vueroute:*} [1297]
          │                    │                │   │  │  │  ╰─ urity/
          │                    │                │   │  │  │          ├─ configuration [1299]
          │                    │                │   │  │  │          │              ╰─ /
          │                    │                │   │  │  │          │                 ├─ api_fuzzing [1298]
          │                    │                │   │  │  │          │                 ├─ corpus_management [1300]
          │                    │                │   │  │  │          │                 ├─ dast [1302]
          │                    │                │   │  │  │          │                 ├─ profile_library [1303]
          │                    │                │   │  │  │          │                 │                ╰─ /dast_s
          │                    │                │   │  │  │          │                 │                         ├─ canner_profiles/
          │                    │                │   │  │  │          │                 │                         │                 ├─ new [1305]
          │                    │                │   │  │  │          │                 │                         │                 ╰─ {id}
          │                    │                │   │  │  │          │                 │                         │                       ╰─ /edit [1304]
          │                    │                │   │  │  │          │                 │                         ╰─ ite_profiles/
          │                    │                │   │  │  │          │                 │                                        ├─ new [1307]
          │                    │                │   │  │  │          │                 │                                        ╰─ {id}
          │                    │                │   │  │  │          │                 │                                              ╰─ /edit [1306]
          │                    │                │   │  │  │          │                 ╰─ sast [1313]
          │                    │                │   │  │  │          ├─ d
          │                    │                │   │  │  │          │  ├─ ashboard [1301]
          │                    │                │   │  │  │          │  ╰─ iscover [1308]
          │                    │                │   │  │  │          ├─ policies [1309]
          │                    │                │   │  │  │          │         ╰─ /
          │                    │                │   │  │  │          │            ├─ new [1311]
          │                    │                │   │  │  │          │            ├─ schema [1312]
          │                    │                │   │  │  │          │            ╰─ {id}
          │                    │                │   │  │  │          │                  ╰─ /edit [1310]
          │                    │                │   │  │  │          ├─ scanned_resources [1314]
          │                    │                │   │  │  │          ╰─ vulnerabilit
          │                    │                │   │  │  │                        ├─ ies/
          │                    │                │   │  │  │                        │     ├─ new [1317]
          │                    │                │   │  │  │                        │     ├─ {id} [1315]
          │                    │                │   │  │  │                        │     │     ╰─ /discussions [1316]
          │                    │                │   │  │  │                        │     ╰─ {vulnerability_id}
          │                    │                │   │  │  │                        │                         ╰─ /notes [1318]
          │                    │                │   │  │  │                        │                                 ╰─ /
          │                    │                │   │  │  │                        │                                    ╰─ {id} [1319]
          │                    │                │   │  │  │                        │                                          ╰─ /toggle_award_emoji [1320]
          │                    │                │   │  │  │                        ╰─ y_report [1321]
          │                    │                │   │  │  ├─ rvice_desk/custom_email [1323]
          │                    │                │   │  │  ╰─ ttings/
          │                    │                │   │  │           ├─ a
          │                    │                │   │  │           │  ├─ ccess_tokens [1325]
          │                    │                │   │  │           │  │             ╰─ /
          │                    │                │   │  │           │  │                ╰─ {id}
          │                    │                │   │  │           │  │                      ╰─ /revoke [1326]
          │                    │                │   │  │           │  ╰─ nalytics [1327]
          │                    │                │   │  │           ├─ repository [1346]
          │                    │                │   │  │           │           ╰─ /
          │                    │                │   │  │           │              ├─ branch_rules [1328]
          │                    │                │   │  │           │              ├─ cleanup [1347]
          │                    │                │   │  │           │              ╰─ deploy_token/create [1348]
          │                    │                │   │  │           ├─ ci_cd [1329]
          │                    │                │   │  │           │      ╰─ /
          │                    │                │   │  │           │         ├─ r
          │                    │                │   │  │           │         │  ├─ eset_
          │                    │                │   │  │           │         │  │      ├─ cache [1330]
          │                    │                │   │  │           │         │  │      ╰─ registration_token [1331]
          │                    │                │   │  │           │         │  ╰─ unner_setup_scripts [1332]
          │                    │                │   │  │           │         ╰─ deploy_token/create [1345]
          │                    │                │   │  │           ├─ integrations [1335]
          │                    │                │   │  │           │             ╰─ /
          │                    │                │   │  │           │                ├─ {integration_id}
          │                    │                │   │  │           │                │                 ╰─ /hook_logs/
          │                    │                │   │  │           │                │                              ╰─ {id} [1333]
          │                    │                │   │  │           │                │                                    ╰─ /retry [1334]
          │                    │                │   │  │           │                ╰─ {id} [1336]
          │                    │                │   │  │           │                      ╰─ /
          │                    │                │   │  │           │                         ├─ edit [1337]
          │                    │                │   │  │           │                         ╰─ test [1338]
          │                    │                │   │  │           ├─ merge_requests [1339]
          │                    │                │   │  │           ├─ operations [1340]
          │                    │                │   │  │           │           ╰─ /reset_
          │                    │                │   │  │           │                    ├─ alerting_token [1341]
          │                    │                │   │  │           │                    ╰─ pagerduty_token [1342]
          │                    │                │   │  │           ├─ packages_and_registries [1343]
          │                    │                │   │  │           │                        ╰─ /cleanup_image_tags [1344]
          │                    │                │   │  │           ╰─ slack [1349]
          │                    │                │   │  │                  ╰─ /
          │                    │                │   │  │                     ├─ edit [1350]
          │                    │                │   │  │                     ╰─ slack_auth [1351]
          │                    │                │   │  ├─ nippets [1352]
          │                    │                │   │  │        ╰─ /
          │                    │                │   │  │           ├─ new [1358]
          │                    │                │   │  │           ├─ {id} [1353]
          │                    │                │   │  │           │     ╰─ /
          │                    │                │   │  │           │        ├─ edit [1354]
          │                    │                │   │  │           │        ├─ mark_as_spam [1355]
          │                    │                │   │  │           │        ├─ raw [1356]
          │                    │                │   │  │           │        ╰─ toggle_award_emoji [1357]
          │                    │                │   │  │           ╰─ {snippet_id}
          │                    │                │   │  │                         ╰─ /raw/
          │                    │                │   │  │                                ╰─ {ref}
          │                    │                │   │  │                                       ╰─ /
          │                    │                │   │  │                                          ╰─ {path:*} [1360]
          │                    │                │   │  ├─ tarrers [1361]
          │                    │                │   │  ├─ ubscriptions [1362]
          │                    │                │   │  │             ╰─ /
          │                    │                │   │  │                ╰─ {id} [1363]
          │                    │                │   │  ╰─ chema/
          │                    │                │   │          ╰─ {branch}
          │                    │                │   │                    ╰─ /
          │                    │                │   │                       ╰─ {filename:*} [1389]
          │                    │                │   ╰─ {noteable_type}
          │                    │                │                    ╰─ /
          │                    │                │                       ╰─ {noteable_id}
          │                    │                │                                      ╰─ /discussions/
          │                    │                │                                                     ╰─ {id} [937]
          │                    │                │                                                           ╰─ /resolve [938]
          │                    │                ├─ r
          │                    │                │  ├─ e
          │                    │                │  │  ├─ store [782]
          │                    │                │  │  ├─ gistry/repository/
          │                    │                │  │  │                   ╰─ {repository_id}
          │                    │                │  │  │                                    ╰─ /tags [1268]
          │                    │                │  │  │                                           ╰─ /
          │                    │                │  │  │                                              ├─ bulk_destroy [1270]
          │                    │                │  │  │                                              ╰─ {id} [1269]
          │                    │                │  │  ╰─ pository [1281]
          │                    │                │  ├─ aw/
          │                    │                │  │    ╰─ {id:*} [1261]
          │                    │                │  ╰─ unner_projects [1285]
          │                    │                │                  ╰─ /
          │                    │                │                     ╰─ {id} [1286]
          │                    │                ├─ alerts/notify [789]
          │                    │                │              ╰─ /
          │                    │                │                 ╰─ {name}
          │                    │                │                         ╰─ /
          │                    │                │                            ╰─ {endpoint_identifier} [790]
          │                    │                ├─ p
          │                    │                │  ├─ rometheus/
          │                    │                │  │           ├─ alerts/
          │                    │                │  │           │        ├─ notify [791]
          │                    │                │  │           │        ╰─ {id}
          │                    │                │  │           │              ╰─ /metrics_dashboard [792]
          │                    │                │  │           ╰─ metrics [1243]
          │                    │                │  │                    ╰─ /
          │                    │                │  │                       ├─ active_common [1246]
          │                    │                │  │                       ├─ new [1247]
          │                    │                │  │                       ├─ validate_query [1248]
          │                    │                │  │                       ╰─ {id} [1244]
          │                    │                │  │                             ╰─ /edit [1245]
          │                    │                │  ╰─ a
          │                    │                │     ├─ ges [1196]
          │                    │                │     │    ╰─ /
          │                    │                │     │       ├─ new [1197]
          │                    │                │     │       ╰─ domains [1198]
          │                    │                │     │                ╰─ /
          │                    │                │     │                   ├─ new [1204]
          │                    │                │     │                   ╰─ {id} [1199]
          │                    │                │     │                         ╰─ /
          │                    │                │     │                            ├─ clean_certificate [1200]
          │                    │                │     │                            ├─ edit [1201]
          │                    │                │     │                            ├─ retry_auto_ssl [1202]
          │                    │                │     │                            ╰─ verify [1203]
          │                    │                │     ╰─ th_locks [1205]
          │                    │                │               ╰─ /
          │                    │                │                  ├─ toggle [1207]
          │                    │                │                  ╰─ {id} [1206]
          │                    │                ├─ b
          │                    │                │  ├─ adges [843]
          │                    │                │  │      ╰─ /
          │                    │                │  │         ╰─ {ref:*}
          │                    │                │  │                  ╰─ /
          │                    │                │  │                     ├─ coverage [844]
          │                    │                │  │                     ╰─ pipeline [845]
          │                    │                │  ├─ l
          │                    │                │  │  ├─ ame/
          │                    │                │  │  │     ╰─ {id:*} [849]
          │                    │                │  │  ╰─ ob/
          │                    │                │  │       ╰─ {id:*} [857]
          │                    │                │  ╰─ uilds [872]
          │                    │                │         ╰─ /
          │                    │                │            ├─ artifacts/
          │                    │                │            │           ╰─ {ref_name_and_path:*} [871]
          │                    │                │            ├─ {build_id}
          │                    │                │            │           ╰─ /artifacts/
          │                    │                │            │                        ├─ browse [866]
          │                    │                │            │                        │       ╰─ /
          │                    │                │            │                        │          ╰─ {path:*} [867]
          │                    │                │            │                        ├─ download [868]
          │                    │                │            │                        ├─ file/
          │                    │                │            │                        │      ╰─ {path:*} [869]
          │                    │                │            │                        ╰─ raw/
          │                    │                │            │                              ╰─ {path:*} [870]
          │                    │                │            ╰─ {id} [873]
          │                    │                │                  ╰─ /raw [874]
          │                    │                ├─ i
          │                    │                │  ├─ nsights [1009]
          │                    │                │  │        ╰─ /query [1010]
          │                    │                │  ╰─ de_terminals [1390]
          │                    │                │                ╰─ /
          │                    │                │                   ├─ check_config [1394]
          │                    │                │                   ╰─ {id} [1391]
          │                    │                │                         ╰─ /
          │                    │                │                            ├─ cancel [1392]
          │                    │                │                            ╰─ retry [1393]
          │                    │                ├─ note
          │                    │                │     ├─ able/
          │                    │                │     │      ╰─ {target_type}
          │                    │                │     │                     ╰─ /
          │                    │                │     │                        ╰─ {target_id}
          │                    │                │     │                                     ╰─ /notes [1181]
          │                    │                │     ╰─ s [1182]
          │                    │                │        ╰─ /
          │                    │                │           ╰─ {id} [1183]
          │                    │                │                 ╰─ /
          │                    │                │                    ├─ delete_attachment [1184]
          │                    │                │                    ├─ outdated_line_change [1185]
          │                    │                │                    ├─ resolve [1186]
          │                    │                │                    ╰─ toggle_award_emoji [1187]
          │                    │                ├─ container_registry [1266]
          │                    │                │                   ╰─ /
          │                    │                │                      ╰─ {id} [1267]
          │                    │                ├─ s
          │                    │                │  ├─ ervice_
          │                    │                │  │        ├─ desk [1322]
          │                    │                │  │        ╰─ ping/web_ide_pipelines_count [1324]
          │                    │                │  ╰─ nippets/
          │                    │                │            ╰─ {id}
          │                    │                │                  ╰─ /raw [1359]
          │                    │                ├─ description_templates/names/
          │                    │                │                             ╰─ {template_type} [1369]
          │                    │                ├─ t
          │                    │                │  ├─ emplates/
          │                    │                │  │          ╰─ {template_type} [1370]
          │                    │                │  │                           ╰─ /
          │                    │                │  │                              ╰─ {key} [1371]
          │                    │                │  ├─ odos [1373]
          │                    │                │  ╰─ ree/
          │                    │                │        ╰─ {id:*} [1378]
          │                    │                ├─ uploads [1381]
          │                    │                │        ╰─ /
          │                    │                │           ├─ authorize [1383]
          │                    │                │           ╰─ {secret}
          │                    │                │                     ╰─ /
          │                    │                │                        ╰─ {filename} [1382]
          │                    │                ╰─ {all:*} [219]
          │                    ╰─ {id} [763]
          │                          ╰─ /
          │                             ├─ a
          │                             │  ├─ ctivity [764]
          │                             │  ╰─ rchive [765]
          │                             ├─ download_export [766]
          │                             ├─ e
          │                             │  ├─ dit [767]
          │                             │  ╰─ xport [768]
          │                             ├─ generate_new_export [769]
          │                             ├─ housekeeping [770]
          │                             ├─ new_issuable_address [771]
          │                             ├─ preview_markdown [772]
          │                             ├─ re
          │                             │   ├─ fs [773]
          │                             │   ╰─ move_
          │                             │          ├─ export [774]
          │                             │          ╰─ fork [775]
          │                             ├─ t
          │                             │  ├─ oggle_star [776]
          │                             │  ╰─ ransfer [777]
          │                             ╰─ un
          │                                 ├─ archive [778]
          │                                 ╰─ foldered_environment_names [779]
          ├─ {repository_path:*}
          │                    ╰─ /
          │                       ├─ git
          │                       │    ├─ -
          │                       │    │  ├─ receive-pack [1444]
          │                       │    │  ╰─ upload-pack [1445]
          │                       │    ╰─ lab-lfs/objects/
          │                       │                      ├─ {oid:*}
          │                       │                      │        ╰─ /
          │                       │                      │           ├─ {size:*}
          │                       │                      │           │         ╰─ /authorize [1471]
          │                       │                      │           ╰─ {size:*} [1470]
          │                       │                      ╰─ {oid:*} [1469]
          │                       ├─ info/
          │                       │      ├─ refs [1446]
          │                       │      ╰─ lfs/
          │                       │            ├─ objects [1451]
          │                       │            │        ╰─ /
          │                       │            │           ├─ batch [1453]
          │                       │            │           ╰─ {oid:*} [1452]
          │                       │            ╰─ locks [1460]
          │                       │                   ╰─ /
          │                       │                      ├─ new [1464]
          │                       │                      ├─ verify [1465]
          │                       │                      ╰─ {id} [1461]
          │                       │                            ╰─ /
          │                       │                               ├─ edit [1462]
          │                       │                               ╰─ unlock [1463]
          │                       ╰─ ssh-upload-pack [1447]
          ╰─ {id:*} [287]
    "###);
}
